-- Adminer 4.7.7 PostgreSQL dump

\connect "recipe";

DROP TABLE IF EXISTS "__diesel_schema_migrations";
CREATE TABLE "public"."__diesel_schema_migrations" (
    "version" character varying(50) NOT NULL,
    "run_on" timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT "__diesel_schema_migrations_pkey" PRIMARY KEY ("version")
) WITH (oids = false);

INSERT INTO "__diesel_schema_migrations" ("version", "run_on") VALUES
('00000000000000',	'2020-09-11 03:20:06.075108'),
('20200416034909',	'2020-09-11 03:20:06.078276'),
('20200419121854',	'2020-09-11 03:20:06.119051');

DROP TABLE IF EXISTS "category";
DROP SEQUENCE IF EXISTS category_id_seq;
CREATE SEQUENCE category_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 START 3 CACHE 1;

CREATE TABLE "public"."category" (
    "id" integer DEFAULT nextval('category_id_seq') NOT NULL,
    "name" character varying NOT NULL,
    CONSTRAINT "category_name_key" UNIQUE ("name"),
    CONSTRAINT "category_pkey" PRIMARY KEY ("id")
) WITH (oids = false);

INSERT INTO "category" ("id", "name") VALUES
(1,	'Desserts'),
(2,	'Plats principaux');

DROP TABLE IF EXISTS "how_to_section";
DROP SEQUENCE IF EXISTS how_to_section_id_seq;
CREATE SEQUENCE how_to_section_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 START 2 CACHE 1;

CREATE TABLE "public"."how_to_section" (
    "id" integer DEFAULT nextval('how_to_section_id_seq') NOT NULL,
    "name" character varying NOT NULL,
    CONSTRAINT "how_to_section_name_key" UNIQUE ("name"),
    CONSTRAINT "how_to_section_pkey" PRIMARY KEY ("id")
) WITH (oids = false);

INSERT INTO "how_to_section" ("id", "name") VALUES
(1,	'');

DROP TABLE IF EXISTS "how_to_step";
DROP SEQUENCE IF EXISTS how_to_step_id_seq;
CREATE SEQUENCE how_to_step_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 START 10 CACHE 1;

CREATE TABLE "public"."how_to_step" (
    "id" integer DEFAULT nextval('how_to_step_id_seq') NOT NULL,
    "name" character varying NOT NULL,
    CONSTRAINT "how_to_step_name_key" UNIQUE ("name"),
    CONSTRAINT "how_to_step_pkey" PRIMARY KEY ("id")
) WITH (oids = false);

INSERT INTO "how_to_step" ("id", "name") VALUES
(1,	'Dans un bol, mélanger la farine, la poudre d’amandes, la poudre à pâte et le sel. Réserver.'),
(2,	'Dans un autre bol, crémer le beurre avec le sucre à glacer, l’eau, la vanille et l’extrait d’amandes au batteur électrique. À basse vitesse ou à la cuillère de bois, incorporer les ingrédients secs.'),
(3,	'Sur un plan de travail, déposer la pâte sur une feuille de papier parchemin ou de papier d’aluminium. Former un rouleau d’environ 5 cm (2 po) de diamètre. Refermer le rouleau en tortillant les deux extrémités du papier d’aluminium. Réfrigérer environ 3 heures ou jusqu’à ce que la pâte soit très ferme au toucher.'),
(4,	'Placer la grille au centre du four. Préchauffer le four à 190 °C (375 °F). Tapisser deux plaques à biscuits de papier parchemin.'),
(5,	'Déballer et placer le rouleau sur un plan de travail. Couper en tranches d’environ 1 cm (½ po) d’épaisseur et les répartir sur les plaques. '),
(6,	'Cuire au four, une plaque à la fois, environ 12 minutes, ou jusqu’à ce que les biscuits soient légèrement dorés. Laisser refroidir sur la plaque.'),
(7,	'Préchauffer l’eau au cuiseur de précision à 68 °C (155 °F).'),
(8,	'Dans un bol, verser le sel et le poivre sur la cuisse de canard. Frotter pendant 1 minute. Placer le tout dans un sac à fermeture hermétique avec le reste des ingrédients.'),
(9,	'Placer dans l’eau préchauffée. Cuire 36 heures. Ajouter de l’eau, au besoin, pour s’assurer que la viande soit toujours submergée.');

DROP TABLE IF EXISTS "ingredient";
DROP SEQUENCE IF EXISTS ingredient_id_seq;
CREATE SEQUENCE ingredient_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 START 16 CACHE 1;

CREATE TABLE "public"."ingredient" (
    "id" integer DEFAULT nextval('ingredient_id_seq') NOT NULL,
    "name" character varying NOT NULL,
    CONSTRAINT "ingredient_name_key" UNIQUE ("name"),
    CONSTRAINT "ingredient_pkey" PRIMARY KEY ("id")
) WITH (oids = false);

INSERT INTO "ingredient" ("id", "name") VALUES
(1,	'375 ml (1 1/2 tasse) de farine tout usage non blanchie'),
(2,	'250 ml (1 tasse) de poudre d’amandes'),
(3,	'5 ml (1 c. à thé) de poudre à pâte'),
(4,	'1 ml (1/4 c. à thé) de sel'),
(5,	'180 ml (3/4 tasse) de beurre non salé, ramolli'),
(6,	'250 ml (1 tasse) de sucre à glacer'),
(7,	'15 ml (1 c. à soupe) d’eau froide'),
(8,	'5 ml (1 c. à thé) d’extrait de vanille'),
(9,	'1 ml (1/4 c. à thé) d’extrait d’amande (facultatif)'),
(10,	'5 ml (1 c. à thé) de fleur de sel (ou de sel casher)"'),
(11,	'1 ml (1/4 c. à thé) de poivre moulu"'),
(12,	'1 cuisse de canard"'),
(13,	'1 gousse d’ail, pelée et légèrement écrasée"'),
(14,	'1 feuille de laurier"'),
(15,	'2 branches de thym"');

DROP TABLE IF EXISTS "keyword";
DROP SEQUENCE IF EXISTS keyword_id_seq;
CREATE SEQUENCE keyword_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 START 17 CACHE 1;

CREATE TABLE "public"."keyword" (
    "id" integer DEFAULT nextval('keyword_id_seq') NOT NULL,
    "name" character varying NOT NULL,
    CONSTRAINT "keyword_name_key" UNIQUE ("name"),
    CONSTRAINT "keyword_pkey" PRIMARY KEY ("id")
) WITH (oids = false);

INSERT INTO "keyword" ("id", "name") VALUES
(1,	'recettes de Noël'),
(2,	'desserts de Noël'),
(3,	'biscuits de Noël'),
(4,	'biscuits sablés au beurre'),
(5,	'recettes de biscuits sablés au beurre'),
(6,	'biscuits'),
(7,	'recettes de biscuits'),
(8,	'sous vide'),
(9,	'recettes avec sous vide'),
(10,	'recettes sous vide'),
(11,	'cuisiner au sous vide'),
(12,	'canard sous vide'),
(13,	'recettes canard'),
(14,	'canard'),
(15,	'cuiseur de précision'),
(16,	'thermocirculateur');

DROP TABLE IF EXISTS "recipe";
DROP SEQUENCE IF EXISTS recipe_id_seq;
CREATE SEQUENCE recipe_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 START 3 CACHE 1;

CREATE TABLE "public"."recipe" (
    "id" integer DEFAULT nextval('recipe_id_seq') NOT NULL,
    "name" character varying NOT NULL,
    "category_id" integer NOT NULL,
    "author" character varying,
    "image" character varying,
    "prep_time" character varying,
    "cook_time" character varying,
    "total_time" character varying,
    "recipe_yield" character varying,
    "description" text NOT NULL,
    "json_ld" text NOT NULL,
    CONSTRAINT "recipe_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "recipe_category_id_fkey" FOREIGN KEY (category_id) REFERENCES category(id) NOT DEFERRABLE
) WITH (oids = false);

INSERT INTO "recipe" ("id", "name", "category_id", "author", "image", "prep_time", "cook_time", "total_time", "recipe_yield", "description", "json_ld") VALUES
(1,	'Biscuits au beurre réfrigérateur',	1,	'Ricardocuisine',	'https://images.ricardocuisine.com/services/recipes/4934.jpg',	'PT20M',	'PT12M',	'PT32M',	'40 biscuits, environ',	'Recette de Ricardo de biscuits au beurre réfrigérateur',	'{
    "name": "Biscuits au beurre réfrigérateur",
    "author": {
        "@type": "Person",
        "name": "Ricardocuisine"
    },
    "image": "https://images.ricardocuisine.com/services/recipes/4934.jpg",
    "datePublished": "2011-05-05T13:40:01+0000",
    "prepTime": "PT20M",
    "cookTime": "PT12M",
    "recipeIngredient": [
        "375 ml (1 1/2 tasse) de farine tout usage non blanchie",
        "250 ml (1 tasse) de poudre d’amandes",
        "5 ml (1 c. à thé) de poudre à pâte",
        "1 ml (1/4 c. à thé) de sel",
        "180 ml (3/4 tasse) de beurre non salé, ramolli",
        "250 ml (1 tasse) de sucre à glacer",
        "15 ml (1 c. à soupe) d’eau froide",
        "5 ml (1 c. à thé) d’extrait de vanille",
        "1 ml (1/4 c. à thé) d’extrait d’amande (facultatif)"
    ],
    "recipeInstructions": [
        {
            "@type": "HowToSection",
            "name": "",
            "itemListElement": [
                {
                    "@type": "HowToStep",
                    "text": "Dans un bol, mélanger la farine, la poudre d’amandes, la poudre à pâte et le sel. Réserver."
                },
                {
                    "@type": "HowToStep",
                    "text": "Dans un autre bol, crémer le beurre avec le sucre à glacer, l’eau, la vanille et l’extrait d’amandes au batteur électrique. À basse vitesse ou à la cuillère de bois, incorporer les ingrédients secs."
                },
                {
                    "@type": "HowToStep",
                    "text": "Sur un plan de travail, déposer la pâte sur une feuille de papier parchemin ou de papier d’aluminium. Former un rouleau d’environ 5 cm (2 po) de diamètre. Refermer le rouleau en tortillant les deux extrémités du papier d’aluminium. Réfrigérer environ 3 heures ou jusqu’à ce que la pâte soit très ferme au toucher."
                },
                {
                    "@type": "HowToStep",
                    "text": "Placer la grille au centre du four. Préchauffer le four à 190 °C (375 °F). Tapisser deux plaques à biscuits de papier parchemin."
                },
                {
                    "@type": "HowToStep",
                    "text": "Déballer et placer le rouleau sur un plan de travail. Couper en tranches d’environ 1 cm (½ po) d’épaisseur et les répartir sur les plaques. "
                },
                {
                    "@type": "HowToStep",
                    "text": "Cuire au four, une plaque à la fois, environ 12 minutes, ou jusqu’à ce que les biscuits soient légèrement dorés. Laisser refroidir sur la plaque."
                }
            ]
        }
    ],
    "recipeYield": "40 biscuits, environ",
    "aggregateRating": {
        "@type": "AggregateRating",
        "ratingValue": 5,
        "ratingCount": 29,
        "bestRating": "5",
        "worstRating": "1"
    },
    "nutrition": null,
    "description": "Recette de Ricardo de biscuits au beurre réfrigérateur",
    "recipeCategory": "Desserts",
    "keywords": "recettes de Noël, desserts de Noël, biscuits de Noël,  biscuits sablés au beurre, recettes de biscuits sablés au beurre, biscuits, recettes de biscuits",
    "totalTime": "PT32M",
    "review": [],
    "video": [],
    "@context": "http://schema.org",
    "@type": "Recipe"
}'),
(2,	'Confit de canard sous vide',	2,	'Ricardocuisine',	'https://images.ricardocuisine.com/services/recipes/8645.jpg',	'PT10M',	'PT36H',	'PT36H10M',	'1 portion',	'Confit de canard sous vide | RICARDO',	'{
    "name": "Confit de canard sous vide",
    "author": {
        "@type": "Person",
        "name": "Ricardocuisine"
    },
    "image": "https://images.ricardocuisine.com/services/recipes/8645.jpg",
    "datePublished": "2019-11-19T18:20:39+0000",
    "prepTime": "PT10M",
    "cookTime": "PT36H",
    "recipeIngredient": [
        "5 ml (1 c. à thé) de fleur de sel (ou de sel casher)",
        "1 ml (1/4 c. à thé) de poivre moulu",
        "1 cuisse de canard",
        "1 gousse d’ail, pelée et légèrement écrasée",
        "1 feuille de laurier",
        "2 branches de thym"
    ],
    "recipeInstructions": [
        {
            "@type": "HowToSection",
            "name": "",
            "itemListElement": [
                {
                    "@type": "HowToStep",
                    "text": "Préchauffer l’eau au cuiseur de précision à 68 °C (155 °F)."
                },
                {
                    "@type": "HowToStep",
                    "text": "Dans un bol, verser le sel et le poivre sur la cuisse de canard. Frotter pendant 1 minute. Placer le tout dans un sac à fermeture hermétique avec le reste des ingrédients."
                },
                {
                    "@type": "HowToStep",
                    "text": "Placer dans l’eau préchauffée. Cuire 36 heures. Ajouter de l’eau, au besoin, pour s’assurer que la viande soit toujours submergée."
                }
            ]
        }
    ],
    "recipeYield": "1 portion",
    "aggregateRating": {
        "@type": "AggregateRating",
        "ratingValue": 5,
        "ratingCount": 8,
        "bestRating": "5",
        "worstRating": "1"
    },
    "nutrition": null,
    "description": "Confit de canard sous vide | RICARDO",
    "recipeCategory": "Plats principaux",
    "keywords": "sous vide, recettes avec sous vide, recettes sous vide, cuisiner au sous vide, canard sous vide, recettes canard, canard, cuiseur de précision, thermocirculateur",
    "totalTime": "PT36H10M",
    "review": [],
    "video": [],
    "@context": "http://schema.org",
    "@type": "Recipe"
}');

DROP TABLE IF EXISTS "recipe_how_to_section";
DROP SEQUENCE IF EXISTS recipe_how_to_section_id_seq;
CREATE SEQUENCE recipe_how_to_section_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 START 3 CACHE 1;

CREATE TABLE "public"."recipe_how_to_section" (
    "id" integer DEFAULT nextval('recipe_how_to_section_id_seq') NOT NULL,
    "recipe_id" integer NOT NULL,
    "how_to_section_id" integer NOT NULL,
    CONSTRAINT "recipe_how_to_section_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "recipe_how_to_section_how_to_section_id_fkey" FOREIGN KEY (how_to_section_id) REFERENCES how_to_section(id) NOT DEFERRABLE,
    CONSTRAINT "recipe_how_to_section_recipe_id_fkey" FOREIGN KEY (recipe_id) REFERENCES recipe(id) NOT DEFERRABLE
) WITH (oids = false);

INSERT INTO "recipe_how_to_section" ("id", "recipe_id", "how_to_section_id") VALUES
(1,	1,	1),
(2,	2,	1);

DROP TABLE IF EXISTS "recipe_how_to_section_how_to_step";
DROP SEQUENCE IF EXISTS recipe_how_to_section_how_to_step_id_seq;
CREATE SEQUENCE recipe_how_to_section_how_to_step_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 START 10 CACHE 1;

CREATE TABLE "public"."recipe_how_to_section_how_to_step" (
    "id" integer DEFAULT nextval('recipe_how_to_section_how_to_step_id_seq') NOT NULL,
    "recipe_how_to_section_id" integer NOT NULL,
    "how_to_step_id" integer NOT NULL,
    CONSTRAINT "recipe_how_to_section_how_to_step_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "recipe_how_to_section_how_to_step_how_to_step_id_fkey" FOREIGN KEY (how_to_step_id) REFERENCES how_to_step(id) NOT DEFERRABLE,
    CONSTRAINT "recipe_how_to_section_how_to_step_recipe_how_to_section_id_fkey" FOREIGN KEY (recipe_how_to_section_id) REFERENCES recipe_how_to_section(id) NOT DEFERRABLE
) WITH (oids = false);

INSERT INTO "recipe_how_to_section_how_to_step" ("id", "recipe_how_to_section_id", "how_to_step_id") VALUES
(1,	1,	1),
(2,	1,	2),
(3,	1,	3),
(4,	1,	4),
(5,	1,	5),
(6,	1,	6),
(7,	2,	7),
(8,	2,	8),
(9,	2,	9);

DROP TABLE IF EXISTS "recipe_ingredient";
DROP SEQUENCE IF EXISTS recipe_ingredient_id_seq;
CREATE SEQUENCE recipe_ingredient_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 START 16 CACHE 1;

CREATE TABLE "public"."recipe_ingredient" (
    "id" integer DEFAULT nextval('recipe_ingredient_id_seq') NOT NULL,
    "recipe_id" integer NOT NULL,
    "ingredient_id" integer NOT NULL,
    CONSTRAINT "recipe_ingredient_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "recipe_ingredient_ingredient_id_fkey" FOREIGN KEY (ingredient_id) REFERENCES ingredient(id) NOT DEFERRABLE,
    CONSTRAINT "recipe_ingredient_recipe_id_fkey" FOREIGN KEY (recipe_id) REFERENCES recipe(id) NOT DEFERRABLE
) WITH (oids = false);

INSERT INTO "recipe_ingredient" ("id", "recipe_id", "ingredient_id") VALUES
(1,	1,	1),
(2,	1,	2),
(3,	1,	3),
(4,	1,	4),
(5,	1,	5),
(6,	1,	6),
(7,	1,	7),
(8,	1,	8),
(9,	1,	9),
(10,	2,	10),
(11,	2,	11),
(12,	2,	12),
(13,	2,	13),
(14,	2,	14),
(15,	2,	15);

DROP TABLE IF EXISTS "recipe_keyword";
DROP SEQUENCE IF EXISTS recipe_keyword_id_seq;
CREATE SEQUENCE recipe_keyword_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 START 17 CACHE 1;

CREATE TABLE "public"."recipe_keyword" (
    "id" integer DEFAULT nextval('recipe_keyword_id_seq') NOT NULL,
    "recipe_id" integer NOT NULL,
    "keyword_id" integer NOT NULL,
    CONSTRAINT "recipe_keyword_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "recipe_keyword_keyword_id_fkey" FOREIGN KEY (keyword_id) REFERENCES keyword(id) NOT DEFERRABLE,
    CONSTRAINT "recipe_keyword_recipe_id_fkey" FOREIGN KEY (recipe_id) REFERENCES recipe(id) NOT DEFERRABLE
) WITH (oids = false);

INSERT INTO "recipe_keyword" ("id", "recipe_id", "keyword_id") VALUES
(1,	1,	1),
(2,	1,	2),
(3,	1,	3),
(4,	1,	4),
(5,	1,	5),
(6,	1,	6),
(7,	1,	7),
(8,	2,	8),
(9,	2,	9),
(10,	2,	10),
(11,	2,	11),
(12,	2,	12),
(13,	2,	13),
(14,	2,	14),
(15,	2,	15),
(16,	2,	16);

-- 2020-09-11 03:24:53.568506+00
