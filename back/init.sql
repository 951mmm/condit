CREATE TABLE IF NOT EXISTS condituser (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    username VARCHAR(50) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    bio TEXT,
    image TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS article (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    body TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMP DEFAULT NOW() NOT NULL,
    author_id UUID NOT NULL
);

CREATE TABLE IF NOT EXISTS comment (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMP DEFAULT NOW() NOT NULL,
    article_id UUID NOT NULL,
    user_id UUID NOT NULL,
    body TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tag (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    article_id UUID NOT NULL
);

CREATE TABLE IF NOT EXISTS following (
    follower_id UUID NOT NULL,
    followee_id UUID NOT NULL,
    PRIMARY KEY(follower_id, followee_id)
);

CREATE TABLE IF NOt EXISTS favoriting (
    article_id UUID NOT NULL,
    follower_id UUID NOT NULL,
    PRIMARY KEY(article_id, follower_id)
);