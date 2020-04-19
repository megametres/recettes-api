INSERT INTO "recipe"
VALUES
  (
    1,
    'Biscuits au beurre r\u00e9frig\u00e9rateur',
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
INSERT INTO "main"."category"("id", "name")
VALUES
  (1, 'Desserts');
INSERT INTO "main"."recipe_category"("id", "recipe_id", "category_id")
VALUES
  (1, 1, 1);
INSERT INTO "main"."ingredient"("id", "name")
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
INSERT INTO "main"."recipe_ingredient"("id", "recipe_id", "ingredient_id")
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
INSERT INTO "main"."how_to_section"("id", "name")
VALUES
  (1, '');
INSERT INTO "main"."recipe_how_to_section"("id", "recipe_id", "how_to_section_id")
VALUES
  (1, 1, 1);
INSERT INTO "main"."how_to_step"("id", "name")
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
INSERT INTO "main"."recipe_how_to_section_how_to_step"(
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
INSERT INTO "main"."keyword"("id", "name")
VALUES
  (1, 'recettes de No\u00ebl'),
  (2, 'desserts de No\u00ebl'),
  (3, 'biscuits de No\u00ebl'),
  (4, 'biscuits sabl\u00e9s au beurre'),
  (5, 'recettes de biscuits sabl\u00e9s au beurre'),
  (6, 'biscuits'),
  (7, 'recettes de biscuits');
INSERT INTO "main"."recipe_keyword"("id", "recipe_id", "keyword_id")
VALUES
  (1, 1, 1),
  (2, 1, 2),
  (3, 1, 3),
  (4, 1, 4),
  (5, 1, 5),
  (6, 1, 6),
  (7, 1, 7);