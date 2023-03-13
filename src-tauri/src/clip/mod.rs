use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sqlite::{State, Value};
use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, Serialize, Deserialize,Default)]
pub struct Clip{
      pub text: String, // the text of the clip
      pub timestamp: i64, // in seconds
      pub id: i64, // the id of the clip
      pub favorite: bool, // if the clip is a favorite
}

#[derive(Debug, Clone, Serialize, Deserialize,Default)]
pub struct Clips{
      pub current_clip: i64, // the id of the current clip
      pub num : i64, // the number of clips in the database
      cached_clips: HashMap<i64,ClipCache>, // the clips that are currently in the cache
}

#[derive(Default)]
pub struct ClipData {
      pub clips: Clips, // the clips
      database_connection: Option<sqlite::Connection>, // the connection to the database
}

pub struct ClipDataMutex {
      pub clip_data: std::sync::Mutex<ClipData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ClipCache{
      pub clip: Clip, // the clip
      pub add_timestamp: i64, // in seconds

      // cache management
      // load the latest config.clips_to_show*2 clips into the cache
      // if the cache is full, remove the oldest clips
      // if a clip that is not in the latest config.clips_to_show*2 clips, and not used in the last config.clip_cache_time(default a day) seconds, remove it from the cache
      // need a daemon thread to do this
}

impl ClipData {
      pub fn get_clip(&mut self, id: i64) -> Option<Clip> {
            // if the clip is in the cache, return it
            let clip_cache = self.clips.cached_clips.get(&id);
            if clip_cache.is_some() {
                  let clip_cache = clip_cache.unwrap();
                  let clip_cache = clip_cache.clone();
                  self.clips.cached_clips.insert(id, ClipCache{
                        clip: clip_cache.clip.clone(),
                        add_timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64,
                  });
                  return Some(clip_cache.clip);
            }

            // if the clip is not in the cache, get it from the database
            let mut statement = self.database_connection.as_ref().unwrap().prepare("SELECT * FROM clips WHERE id = ?").unwrap();
            statement.bind((1,id)).unwrap();
            while let Ok(State::Row) = statement.next() {
                  let text = statement.read::<String,_>("text");
                  if text.is_err() {
                        return None;
                  }

                  let timestamp = statement.read::<i64,_>("timestamp");
                  if timestamp.is_err() {
                        return None;
                  }

                  let id = statement.read::<i64,_>("id");
                  if id.is_err() {
                        return None;
                  }
                  let id = id.unwrap();

                  let favorite = statement.read::<i64,_>("favorite");
                  if favorite.is_err() {
                        return None;
                  }
                  let favorite = favorite.unwrap() == 1;

                  let clip = Clip{
                        text: text.unwrap(),
                        timestamp: timestamp.unwrap(),
                        id,
                        favorite,
                  };

                  self.clips.cached_clips.insert(id, ClipCache{
                        clip: clip.clone(),
                        add_timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64,
                  });
            }

            // if the clip is not in the database, return None
            None
      }

      pub fn get_current_clip(&mut self) -> Option<Clip> {
            self.get_clip(self.clips.current_clip)
      }

      pub fn delete_clip(&mut self, id: i64) -> Result<(),String> {
            // delete a clip from the database and the cache

            // first delete in cache
            self.clips.cached_clips.remove(&id);

            // delete in database
            let mut statement = self.database_connection.as_ref().unwrap().prepare("DELETE FROM clips WHERE id = ?").unwrap();
            statement.bind((1,id)).unwrap();
            if let Ok(State::Done) = statement.next() {
                  self.clips.num -= 1;
                  return Ok(());
            }

            Err("Failed to delete clip".to_string())
      }

      pub fn new_clip(&mut self, text: String) -> Result<i64,String> {
            // create a new clip in the database and return the id of the new clip

            let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64;

            let mut statement = self.database_connection.as_ref().unwrap().prepare("INSERT INTO clips (text, timestamp, favorite) VALUES (?, ?, ?)").unwrap();
            statement.bind::<&[(_, Value)]>(&[
                  (1, text.clone().into()),
                  (2, timestamp.into()),
                  (3, 0.into()),
            ][..]).unwrap();
            if let Ok(State::Done) = statement.next() {
                  // try to get the id of the new clip by searching for the clip with the same timestamp
                  let mut statement = self.database_connection.as_ref().unwrap().prepare("SELECT * FROM clips WHERE timestamp = ?").unwrap();
                  statement.bind((1,timestamp)).unwrap();
                  while let Ok(State::Row) = statement.next() {
                        let id = statement.read::<i64,_>("id");
                        if id.is_err() {
                              return Err("Failed to get id of new clip".to_string());
                        }
                        let id = id.unwrap();

                        let clip = Clip{
                              text,
                              timestamp,
                              id,
                              favorite: false,
                        };

                        self.clips.cached_clips.insert(id, ClipCache{
                              clip: clip,
                              add_timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64,
                        });

                        self.clips.num += 1;
                        return Ok(id);
                  }
            }

            Err("Failed to create new clip".to_string())

      }

      pub fn toggle_favorite_clip(&mut self, _id: i64) -> Result<bool,String> {
            // toggle the favorite status of a clip
            // if the clip is not in the cache, return an error
            // return the new favorite status


            !todo!("toggle_favorite_clip")
      }
}

pub fn init_database_connection(app: &AppHandle) -> Result<(), String> {
      // get the app data dir
      let app_data_dir = app.path_resolver().app_data_dir();
      if app_data_dir.is_none() {
            return Err("Failed to get app data dir".to_string());
      }
      let mut app_data_dir = app_data_dir.unwrap();

      // if the app data dir does not exist, create it
      if app_data_dir.exists() == false {
            if let Err(_) = std::fs::create_dir_all(&app_data_dir) {
                  return Err("Failed to create app data dir".to_string());
            }
      }

      // create the database dir if it does not exist
      app_data_dir.push("database");
      let database_dir = app_data_dir;
      
      let connection = sqlite::open(database_dir.as_path());
      if connection.is_err() {
            return Err("Failed to open database".to_string());
      }

      let connection = connection.unwrap();

      // create the clips table if it does not exist
      let mut statement = connection.prepare("CREATE TABLE IF NOT EXISTS clips (id INTEGER PRIMARY KEY AUTOINCREMENT, text TEXT, timestamp INTEGER, favorite INTEGER)").unwrap();
      if let Ok(State::Done) = statement.next() {
            let clip_data_mutex = app.state::<ClipDataMutex>();
            let mut clip_data = clip_data_mutex.clip_data.lock().unwrap();
            drop(statement);
            clip_data.database_connection = Some(connection);

            // TODO init the cache daemon

            return Ok(());
      }

      return Err("Failed to create clips table".to_string());
}