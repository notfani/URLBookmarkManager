CREATE TABLE categories (
                            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                            name VARCHAR NOT NULL,
                            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                            updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE bookmarks (
                           id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                           title VARCHAR NOT NULL,
                           url VARCHAR NOT NULL,
                           description TEXT,
                           category_id UUID NOT NULL REFERENCES categories(id),
                           created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                           updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('categories');
SELECT diesel_manage_updated_at('bookmarks');