sea-orm-cli migrate init

# Include migration crate

sea-orm-cli generate migration -o migration


sea-orm-cli migrate generate create_video_table
sea-orm-cli migrate generate create_subtitle_table


sea-orm-cli migrate up

cargo new entity --lib


sea-orm-cli generate entity \
  -u postgres://postgres:12345678@127.0.0.1:5432/video_editing_rust \
  -o src/entities