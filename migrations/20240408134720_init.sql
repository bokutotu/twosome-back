CREATE TABLE IF NOT EXISTS users (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name text NOT NULL
);

CREATE TABLE IF NOT EXISTS groups (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name text NOT NULL
);

CREATE TABLE IF NOT EXISTS user_groups (
  user_id uuid NOT NULL,
  group_id uuid NOT NULL,
  PRIMARY KEY (user_id, group_id),
  FOREIGN KEY (user_id) REFERENCES users (id),
  FOREIGN KEY (group_id) REFERENCES groups (id)
);

CREATE TABLE IF NOT EXISTS post_url (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id uuid NOT NULL,
  group_id uuid NOT NULL,
  content text NOT NULL,
  status text NOT NULL,
  created_at timestamp NOT NULL DEFAULT now(),
  FOREIGN KEY (user_id) REFERENCES users (id),
  FOREIGN KEY (group_id) REFERENCES groups (id)
);

CREATE TABLE IF NOT EXISTS post_image (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id uuid NOT NULL,
  group_id uuid NOT NULL,
  content text NOT NULL,
  created_at timestamp NOT NULL DEFAULT now(),
  FOREIGN KEY (user_id) REFERENCES users (id),
  FOREIGN KEY (group_id) REFERENCES groups (id)
);

CREATE TABLE IF NOT EXISTS post_video (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id uuid NOT NULL,
  group_id uuid NOT NULL,
  content text NOT NULL,
  created_at timestamp NOT NULL DEFAULT now(),
  FOREIGN KEY (user_id) REFERENCES users (id),
  FOREIGN KEY (group_id) REFERENCES groups (id)
);
