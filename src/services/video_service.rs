use crate::entities::{ video, subtitle};
use crate::library::ffmpeg::{get_video_duration_in_seconds, trim_video, add_subtitles};
use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set};
use chrono::Utc;
use std::io;
use std::path::Path;
use anyhow::Result;
use tokio::fs;
use std::path::PathBuf;


pub struct VideoService {
    pub db: DatabaseConnection,
}

impl VideoService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn upload_video(
        &self,
        file_path: &str,
        original_name: &str,
        file_size: i64,
    ) -> Result<video::Model, io::Error> {
        let duration = get_video_duration_in_seconds(file_path).await.ok();
        let now = Utc::now();
        let active = video::ActiveModel {
            original_name: Set(original_name.to_string()),
            file_path: Set(file_path.to_string()),
            file_size: Set(file_size),
            duration: Set(duration),
            status: Set("uploaded".to_string()),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(&self.db).await.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?)
    }

    pub async fn trim_video(
        &self,
        id: i32,
        start: &str,
        end: &str,
    ) -> Result<video::Model, io::Error> {
        let video = video::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Video not found"))?;

        let trimmed_path = video.file_path.replace(
            Path::new(&video.file_path)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or(""),
            &format!("_trimmed.{}", Path::new(&video.file_path).extension().and_then(|e| e.to_str()).unwrap_or("mp4"))
        );

        let clip_length = (end.parse::<f64>().unwrap_or(0.0) - start.parse::<f64>().unwrap_or(0.0)).to_string();

        trim_video(&video.file_path, &trimmed_path, start, &clip_length).await?;

        let new_duration = get_video_duration_in_seconds(&trimmed_path).await.unwrap_or(clip_length.parse().unwrap_or(0.0));

        let mut active: video::ActiveModel = video.into();
        active.file_path = Set(trimmed_path);
        active.status = Set("trimmed".to_string());
        active.duration = Set(Some(new_duration));
        active.updated_at = Set(Utc::now().into());

        Ok(active.update(&self.db).await.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?)
    }

    pub async fn add_subtitles(
        &self,
        id: i32,
        subtitle_text: &str,
        start: &str,
        end: &str,
    ) -> Result<video::Model, io::Error> {
        let video = video::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Video not found"))?;

        let subtitled_path = video.file_path.replace(
            Path::new(&video.file_path)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or(""),
            &format!("_subtitled.{}", Path::new(&video.file_path).extension().and_then(|e| e.to_str()).unwrap_or("mp4"))
        );

        add_subtitles(&video.file_path, &subtitled_path, subtitle_text, start, end).await?;

        let start_time = start.parse::<f64>().unwrap_or(0.0);
        let end_time = end.parse::<f64>().unwrap_or(0.0);

        let subtitle_active = subtitle::ActiveModel {
            video_id: Set(video.id),
            text: Set(subtitle_text.to_string()),
            start_time: Set(start_time),
            end_time: Set(end_time),
            created_at: Set(Utc::now().into()),
            ..Default::default()
        };
        subtitle_active.insert(&self.db).await.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        // let duration = video.duration.or_else(|| get_video_duration_in_seconds(&subtitled_path).await.ok());
        let duration = if video.duration.is_some() {
            video.duration
        } else {
            get_video_duration_in_seconds(&subtitled_path).await.ok()
        };

        let mut active: video::ActiveModel = video.into();
        active.file_path = Set(subtitled_path);
        active.status = Set("subtitled".to_string());
        active.duration = Set(duration);
        active.updated_at = Set(Utc::now().into());

        Ok(active.update(&self.db).await.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?)
    }

    pub async fn render_video(
        &self,
        id: i32,
    ) -> Result<video::Model, io::Error> {
        let video = video::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Video not found"))?;

        let rendered_path = video.file_path.replace(
            Path::new(&video.file_path)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or(""),
            &format!("_rendered.{}", Path::new(&video.file_path).extension().and_then(|e| e.to_str()).unwrap_or("mp4"))
        );

        tokio::fs::copy(&video.file_path, &rendered_path).await?;

        let mut active: video::ActiveModel = video.into();
        active.file_path = Set(rendered_path);
        active.status = Set("rendered".to_string());
        active.updated_at = Set(Utc::now().into());

        Ok(active.update(&self.db).await.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?)
    }

    pub async fn get_video_download_path(
    &self,
    id: i32,
) -> Result<String, io::Error> {
    let video = video::Entity::find_by_id(id)
        .one(&self.db)
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Video not found"))?;

    if video.status != "rendered" {
        return Err(io::Error::new(io::ErrorKind::Other, "Video must be rendered before download"));
    }

    let file_path = PathBuf::from(&video.file_path);

    // Use async metadata check to see if file exists
    if fs::metadata(&file_path).await.is_err() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Rendered video file not found on disk"));
    }

    // Optionally canonicalize path (blocking), but only if you really need it
    // let full_path = fs::canonicalize(&file_path).await.unwrap_or(file_path);

    Ok(file_path.to_string_lossy().to_string())
}

}