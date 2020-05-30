INSERT INTO "category"("id", "name")
VALUES
  (1, 'Desserts');
INSERT INTO "recipe"
VALUES
  (
    1,
    'Biscuits au beurre r\u00e9frig\u00e9rateur',
    1,
    'Ricardocuisine',
    'https://images.ricardocuisine.com/services/recipes/4934.jpg',
    'PT20M',
    'PT12M',
    'PT32M',
    '40 biscuits, environ',
    'Recette de Ricardo de biscuits au beurre r\u00e9frig\u00e9rateur',
    '{
    "name": "Biscuits au beurre r\u00e9frig\u00e9rateur",
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
        "250 ml (1 tasse) de poudre d\u2019amandes",
        "5 ml (1 c. \u00e0 th\u00e9) de poudre \u00e0 p\u00e2te",
        "1 ml (1/4 c. \u00e0 th\u00e9) de sel",
        "180 ml (3/4 tasse) de beurre non sal\u00e9, ramolli",
        "250 ml (1 tasse) de sucre \u00e0 glacer",
        "15 ml (1 c. \u00e0 soupe) d\u2019eau froide",
        "5 ml (1 c. \u00e0 th\u00e9) d\u2019extrait de vanille",
        "1 ml (1/4 c. \u00e0 th\u00e9) d\u2019extrait d\u2019amande (facultatif)"
    ],
    "recipeInstructions": [
        {
            "@type": "HowToSection",
            "name": "",
            "itemListElement": [
                {
                    "@type": "HowToStep",
                    "text": "Dans un bol, m\u00e9langer la farine, la poudre d\u2019amandes, la poudre \u00e0 p\u00e2te et le sel. R\u00e9server."
                },
                {
                    "@type": "HowToStep",
                    "text": "Dans un autre bol, cr\u00e9mer le beurre avec le sucre \u00e0 glacer, l\u2019eau, la vanille et l\u2019extrait d\u2019amandes au batteur \u00e9lectrique. \u00c0 basse vitesse ou \u00e0 la cuill\u00e8re de bois, incorporer les ingr\u00e9dients secs."
                },
                {
                    "@type": "HowToStep",
                    "text": "Sur un plan de travail, d\u00e9poser la p\u00e2te sur une feuille de papier parchemin ou de papier d\u2019aluminium. Former un rouleau d\u2019environ 5\u00a0cm (2 po) de diam\u00e8tre. Refermer le rouleau en tortillant les deux extr\u00e9mit\u00e9s du papier d\u2019aluminium. R\u00e9frig\u00e9rer environ 3 heures ou jusqu\u2019\u00e0 ce que la p\u00e2te soit tr\u00e8s ferme au toucher."
                },
                {
                    "@type": "HowToStep",
                    "text": "Placer la grille au centre du four. Pr\u00e9chauffer le four \u00e0 190\u00a0\u00b0C (375 \u00b0F). Tapisser deux plaques \u00e0 biscuits de papier parchemin."
                },
                {
                    "@type": "HowToStep",
                    "text": "D\u00e9baller et placer le rouleau sur un plan de travail. Couper en tranches d\u2019environ 1 cm (\u00bd po) d\u2019\u00e9paisseur et les r\u00e9partir sur les plaques. "
                },
                {
                    "@type": "HowToStep",
                    "text": "Cuire au four, une plaque \u00e0 la fois, environ 12 minutes, ou jusqu\u2019\u00e0 ce que les biscuits soient l\u00e9g\u00e8rement dor\u00e9s. Laisser refroidir sur la plaque."
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
    "description": "Recette de Ricardo de biscuits au beurre r\u00e9frig\u00e9rateur",
    "recipeCategory": "Desserts",
    "keywords": "recettes de No\u00ebl, desserts de No\u00ebl, biscuits de No\u00ebl,  biscuits sabl\u00e9s au beurre, recettes de biscuits sabl\u00e9s au beurre, biscuits, recettes de biscuits",
    "totalTime": "PT32M",
    "review": [],
    "video": [],
    "@context": "http://schema.org",
    "@type": "Recipe"
}'
  );
INSERT INTO "ingredient"("id", "name")
VALUES
  (
    1,
    '375 ml (1 1/2 tasse) de farine tout usage non blanchie'
  ),
  (2, '250 ml (1 tasse) de poudre d\u2019amandes'),
  (
    3,
    '5 ml (1 c. \u00e0 th\u00e9) de poudre \u00e0 p\u00e2te'
  ),
  (4, '1 ml (1/4 c. \u00e0 th\u00e9) de sel'),
  (
    5,
    '180 ml (3/4 tasse) de beurre non sal\u00e9, ramolli'
  ),
  (6, '250 ml (1 tasse) de sucre \u00e0 glacer'),
  (7, '15 ml (1 c. \u00e0 soupe) d\u2019eau froide'),
  (
    8,
    '5 ml (1 c. \u00e0 th\u00e9) d\u2019extrait de vanille'
  ),
  (
    9,
    '1 ml (1/4 c. \u00e0 th\u00e9) d\u2019extrait d\u2019amande (facultatif)'
  );
INSERT INTO "recipe_ingredient"("id", "recipe_id", "ingredient_id")
VALUES
  (1, 1, 1),
  (2, 1, 2),
  (3, 1, 3),
  (4, 1, 4),
  (5, 1, 5),
  (6, 1, 6),
  (7, 1, 7),
  (8, 1, 8),
  (9, 1, 9);
INSERT INTO "how_to_section"("id", "name")
VALUES
  (1, '');
INSERT INTO "recipe_how_to_section"("id", "recipe_id", "how_to_section_id")
VALUES
  (1, 1, 1);
INSERT INTO "how_to_step"("id", "name")
VALUES
  (
    1,
    'Dans un bol, m\u00e9langer la farine, la poudre d\u2019amandes, la poudre \u00e0 p\u00e2te et le sel. R\u00e9server.'
  ),
  (
    2,
    'Dans un autre bol, cr\u00e9mer le beurre avec le sucre \u00e0 glacer, l\u2019eau, la vanille et l\u2019extrait d\u2019amandes au batteur \u00e9lectrique. \u00c0 basse vitesse ou \u00e0 la cuill\u00e8re de bois, incorporer les ingr\u00e9dients secs.'
  ),
  (
    3,
    'Sur un plan de travail, d\u00e9poser la p\u00e2te sur une feuille de papier parchemin ou de papier d\u2019aluminium. Former un rouleau d\u2019environ 5\u00a0cm (2 po) de diam\u00e8tre. Refermer le rouleau en tortillant les deux extr\u00e9mit\u00e9s du papier d\u2019aluminium. R\u00e9frig\u00e9rer environ 3 heures ou jusqu\u2019\u00e0 ce que la p\u00e2te soit tr\u00e8s ferme au toucher.'
  ),
  (
    4,
    'Placer la grille au centre du four. Pr\u00e9chauffer le four \u00e0 190\u00a0\u00b0C (375 \u00b0F). Tapisser deux plaques \u00e0 biscuits de papier parchemin.'
  ),
  (
    5,
    'D\u00e9baller et placer le rouleau sur un plan de travail. Couper en tranches d\u2019environ 1 cm (\u00bd po) d\u2019\u00e9paisseur et les r\u00e9partir sur les plaques. '
  ),
  (
    6,
    'Cuire au four, une plaque \u00e0 la fois, environ 12 minutes, ou jusqu\u2019\u00e0 ce que les biscuits soient l\u00e9g\u00e8rement dor\u00e9s. Laisser refroidir sur la plaque.'
  );
INSERT INTO "recipe_how_to_section_how_to_step"(
    "id",
    "recipe_how_to_section_id",
    "how_to_step_id"
  )
VALUES
  (1, 1, 1),
  (2, 1, 2),
  (3, 1, 3),
  (4, 1, 4),
  (5, 1, 5),
  (6, 1, 6);
INSERT INTO "keyword"("id", "name")
VALUES
  (1, 'recettes de No\u00ebl'),
  (2, 'desserts de No\u00ebl'),
  (3, 'biscuits de No\u00ebl'),
  (4, 'biscuits sabl\u00e9s au beurre'),
  (5, 'recettes de biscuits sabl\u00e9s au beurre'),
  (6, 'biscuits'),
  (7, 'recettes de biscuits');
INSERT INTO "recipe_keyword"("id", "recipe_id", "keyword_id")
VALUES
  (1, 1, 1),
  (2, 1, 2),
  (3, 1, 3),
  (4, 1, 4),
  (5, 1, 5),
  (6, 1, 6),
  (7, 1, 7);
INSERT INTO "category"("id", "name")
VALUES
  (2, 'Plats principaux');
INSERT INTO "recipe"
VALUES
  (
    2,
    'Confit de canard sous vide',
    2,
    'Ricardocuisine',
    'https://images.ricardocuisine.com/services/recipes/8645.jpg',
    'PT10M',
    'PT36H',
    'PT36H10M',
    '1 portion',
    'Confit de canard sous vide | RICARDO',
    '{
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
        "5 ml (1 c. \u00e0 th\u00e9) de fleur de sel (ou de sel casher)",
        "1 ml (1/4 c. \u00e0 th\u00e9) de poivre moulu",
        "1 cuisse de canard",
        "1 gousse d\u2019ail, pel\u00e9e et l\u00e9g\u00e8rement \u00e9cras\u00e9e",
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
                    "text": "Pr\u00e9chauffer l\u2019eau au cuiseur de pr\u00e9cision \u00e0 68\u00a0\u00b0C (155\u00a0\u00b0F)."
                },
                {
                    "@type": "HowToStep",
                    "text": "Dans un bol, verser le sel et le poivre sur la cuisse de canard. Frotter pendant 1 minute. Placer le tout dans un sac \u00e0 fermeture herm\u00e9tique avec le reste des ingr\u00e9dients."
                },
                {
                    "@type": "HowToStep",
                    "text": "Placer dans l\u2019eau pr\u00e9chauff\u00e9e. Cuire 36 heures. Ajouter de l\u2019eau, au besoin, pour s\u2019assurer que la viande soit toujours submerg\u00e9e."
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
    "keywords": "sous vide, recettes avec sous vide, recettes sous vide, cuisiner au sous vide, canard sous vide, recettes canard, canard, cuiseur de pr\u00e9cision, thermocirculateur",
    "totalTime": "PT36H10M",
    "review": [],
    "video": [],
    "@context": "http://schema.org",
    "@type": "Recipe"
}'
  );
INSERT INTO "ingredient"("id", "name")
VALUES
  (
    10,
    '5 ml (1 c. \u00e0 th\u00e9) de fleur de sel (ou de sel casher)"'
  ),
  (
    11,
    '1 ml (1/4 c. \u00e0 th\u00e9) de poivre moulu"'
  ),
  (12, '1 cuisse de canard"'),
  (
    13,
    '1 gousse d\u2019ail, pel\u00e9e et l\u00e9g\u00e8rement \u00e9cras\u00e9e"'
  ),
  (14, '1 feuille de laurier"'),
  (15, '2 branches de thym"');
INSERT INTO "recipe_ingredient"("id", "recipe_id", "ingredient_id")
VALUES
  (10, 2, 10),
  (11, 2, 11),
  (12, 2, 12),
  (13, 2, 13),
  (14, 2, 14),
  (15, 2, 15);
INSERT INTO "recipe_how_to_section"("id", "recipe_id", "how_to_section_id")
VALUES
  (2, 2, 1);
INSERT INTO "how_to_step"("id", "name")
VALUES
  (
    7,
    'Pr\u00e9chauffer l\u2019eau au cuiseur de pr\u00e9cision \u00e0 68\u00a0\u00b0C (155\u00a0\u00b0F).'
  ),
  (
    8,
    'Dans un bol, verser le sel et le poivre sur la cuisse de canard. Frotter pendant 1 minute. Placer le tout dans un sac \u00e0 fermeture herm\u00e9tique avec le reste des ingr\u00e9dients.'
  ),
  (
    9,
    'Placer dans l\u2019eau pr\u00e9chauff\u00e9e. Cuire 36 heures. Ajouter de l\u2019eau, au besoin, pour s\u2019assurer que la viande soit toujours submerg\u00e9e.'
  );
INSERT INTO "recipe_how_to_section_how_to_step"(
    "id",
    "recipe_how_to_section_id",
    "how_to_step_id"
  )
VALUES
  (7, 2, 7),
  (8, 2, 8),
  (9, 2, 9);
INSERT INTO "keyword"("id", "name")
VALUES
  (8, 'sous vide'),
  (9, 'recettes avec sous vide'),
  (10, 'recettes sous vide'),
  (11, 'cuisiner au sous vide'),
  (12, 'canard sous vide'),
  (13, 'recettes canard'),
  (14, 'canard'),
  (15, 'cuiseur de pr\u00e9cision'),
  (16, 'thermocirculateur');
INSERT INTO "recipe_keyword"("id", "recipe_id", "keyword_id")
VALUES
  (8, 2, 8),
  (9, 2, 9),
  (10, 2, 10),
  (11, 2, 11),
  (12, 2, 12),
  (13, 2, 13),
  (14, 2, 14),
  (15, 2, 15),
  (16, 2, 16);