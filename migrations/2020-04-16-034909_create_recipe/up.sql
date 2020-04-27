CREATE TABLE "recipe" (
  "id" SERIAL PRIMARY KEY,
  "name" VARCHAR NOT NULL,
  "author" VARCHAR NOT NULL,
  "image" VARCHAR NOT NULL,
  "prep_time" VARCHAR NOT NULL,
  "cook_time" VARCHAR NOT NULL,
  "total_time" VARCHAR NOT NULL,
  "recipe_yield" VARCHAR NOT NULL,
  "description" TEXT NOT NULL,
  "json_ld" TEXT NOT NULL
);
CREATE TABLE "category" (
  "id" SERIAL PRIMARY KEY,
  "name" VARCHAR NOT NULL
);
CREATE TABLE "recipe_category" (
  "id" SERIAL PRIMARY KEY,
  "recipe_id" INTEGER NOT NULL,
  "category_id" INTEGER NOT NULL,
  FOREIGN KEY("recipe_id") REFERENCES recipe("id"),
  FOREIGN KEY("category_id") REFERENCES category("id")
);
CREATE TABLE "ingredient" (
  "id" SERIAL PRIMARY KEY,
  "name" VARCHAR NOT NULL
);
CREATE TABLE "recipe_ingredient" (
  "id" SERIAL PRIMARY KEY,
  "recipe_id" INTEGER NOT NULL,
  "ingredient_id" INTEGER NOT NULL,
  FOREIGN KEY("recipe_id") REFERENCES recipe("id"),
  FOREIGN KEY("ingredient_id") REFERENCES ingredient("id")
);
CREATE TABLE "how_to_section" (
  "id" SERIAL PRIMARY KEY,
  "name" VARCHAR NOT NULL
);
CREATE TABLE "recipe_how_to_section" (
  "id" SERIAL PRIMARY KEY,
  "recipe_id" INTEGER NOT NULL,
  "how_to_section_id" INTEGER NOT NULL,
  FOREIGN KEY("recipe_id") REFERENCES recipe("id"),
  FOREIGN KEY("how_to_section_id") REFERENCES how_to_section("id")
);
CREATE TABLE "how_to_step" (
  "id" SERIAL PRIMARY KEY,
  "name" text NOT NULL
);
CREATE TABLE "recipe_how_to_section_how_to_step" (
  "id" SERIAL PRIMARY KEY,
  "recipe_how_to_section_id" INTEGER NOT NULL,
  "how_to_step_id" INTEGER NOT NULL,
  FOREIGN KEY("recipe_how_to_section_id") REFERENCES recipe_how_to_section("id"),
  FOREIGN KEY("how_to_step_id") REFERENCES how_to_step("id")
);
CREATE TABLE "keyword" (
  "id" SERIAL PRIMARY KEY,
  "name" VARCHAR NOT NULL
);
CREATE TABLE "recipe_keyword" (
  "id" SERIAL PRIMARY KEY,
  "recipe_id" INTEGER NOT NULL,
  "keyword_id" INTEGER NOT NULL,
  FOREIGN KEY("recipe_id") REFERENCES recipe("id"),
  FOREIGN KEY("keyword_id") REFERENCES keyword("id")
);