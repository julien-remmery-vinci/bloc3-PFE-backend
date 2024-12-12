--Companies
-- Colruyt Group
INSERT INTO pfe.companies (company_name, company_number, legal_form, office_address, website, nace_code, nb_workers, revenue, dispute)
VALUES (
    'Colruyt Group',
    'BE0400378485',
    'SA',
    'Edingensesteenweg 196, 1500 Halle, Belgium',
    'https://www.colruytgroup.com',
    '47.111',
    32000,
    9900000000,
    false
);

-- AB InBev
INSERT INTO pfe.companies (company_name, company_number, legal_form, office_address, website, nace_code, nb_workers, revenue, dispute)
VALUES (
    'Anheuser-Busch InBev',
    'BE0417497106',
    'SA',
    'Boulevard Industriel 21, 1070 Anderlecht, Belgium',
    'https://www.ab-inbev.com',
    '11.050',
    169000,
    54304000000,
    false
);

-- Proximus
INSERT INTO pfe.companies (company_name, company_number, legal_form, office_address, website, nace_code, nb_workers, revenue, dispute)
VALUES (
    'Proximus',
    'BE0202239951',
    'SA',
    'Boulevard du Roi Albert II 27, 1030 Brussels, Belgium',
    'https://www.proximus.be',
    '61.100',
    11532,
    5578000000,
    false
);

--users
INSERT INTO pfe.users (firstname, lastname, login, password, role, company_id)
VALUES
    ('John', 'Doe', 'johndoe', '$2a$10$2PYC2hW.wb9q5mf.xpL6IOi3C03eH3OKYZYOtqtGNMFAJBeI6YLWe', 'user', 1),
    ('Jane', 'Doe', 'janedoe', '$2a$10$2PYC2hW.wb9q5mf.xpL6IOi3C03eH3OKYZYOtqtGNMFAJBeI6YLWe', 'user', 2),
    ('Alice', 'Smith', 'alicesmith', '$2a$10$2PYC2hW.wb9q5mf.xpL6IOi3C03eH3OKYZYOtqtGNMFAJBeI6YLWe', 'user', 3);

-- Insertion des utilisateurs de test
INSERT INTO pfe.users (firstname, lastname, login, password, role, company_id)
VALUES 
('dev', 'quidev', 'dev', '$2a$10$2PYC2hW.wb9q5mf.xpL6IOi3C03eH3OKYZYOtqtGNMFAJBeI6YLWe', 'user', 1),
('user1', 'example', 'user1@example.com', '$2a$10$2PYC2hW.wb9q5mf.xpL6IOi3C03eH3OKYZYOtqtGNMFAJBeI6YLWe', 'user', 1),
('user2', 'example', 'user2@example.com', '$2a$10$2PYC2hW.wb9q5mf.xpL6IOi3C03eH3OKYZYOtqtGNMFAJBeI6YLWe', 'user', 2),
('user3', 'example', 'user3@example.com', '$2a$10$2PYC2hW.wb9q5mf.xpL6IOi3C03eH3OKYZYOtqtGNMFAJBeI6YLWe', 'user', 3),
('admin', 'example', 'admin@example.com', '$2a$10$2PYC2hW.wb9q5mf.xpL6IOi3C03eH3OKYZYOtqtGNMFAJBeI6YLWe', 'admin', null);


--templates
INSERT INTO pfe.templates (value)
VALUES
('ALL'),
('OWNED FACILITY'),
('WORKERS'),
('PRODUITS'),
('FACILITY');

INSERT INTO pfe.template_company (company_id, template_id)
VALUES
    (1,1),
    (2,1),
    (3,1);

--forms
INSERT INTO pfe.forms (company_id, type, status)
VALUES
    (1, 'ESG', 'PENDING'),
    (2, 'ESG', 'SUBMITTED'),
    (3, 'ESG', 'VALIDATED');

INSERT INTO pfe.template_form (form_id, template_id)
VALUES
    (1,1),
    (2,1),
    (3,1);


-- Insertion d'exemples de onboarding
INSERT INTO pfe.onboarding (
    firstname,
    lastname,
    email,
    password,
    position,
    company_name,
    company_number,
    legal_form,
    office_address,
    website,
    nace_code,
    revenue,
    franchise,
    nb_workers,
    dispute,
    honor_engagement,
    grant_application,
    grant_application_partner,
    something_else,
    comment,
    is_owner,
    offers_services,
    sells_products,
    status
) VALUES (
    'Alice',
    'Smith',
    'alice.smith@example.com',
    'securepassword123',
    'Manager',
    'GreenTech Solutions',
    'BE9876543210',
    'LLC',
    '456 Innovation Lane, Paris, France',
    'https://www.greentech.com',
    '6202',
    2000000.50,
    false,
    25,
    false,
    true,
    true,
    'GrantExperts Ltd.',
    'Example data for approved entry.',
    'Excited to start!',
    true,
    true,
    false,
    'ACCEPTED'
);

INSERT INTO pfe.onboarding (
    firstname,
    lastname,
    email,
    password,
    position,
    company_name,
    company_number,
    legal_form,
    office_address,
    website,
    nace_code,
    revenue,
    franchise,
    nb_workers,
    dispute,
    honor_engagement,
    grant_application,
    grant_application_partner,
    something_else,
    comment,
    is_owner,
    offers_services,
    sells_products,
    status
) VALUES (
    'Bob',
    'Johnson',
    'bob.johnson@example.com',
    'mypassword456',
    'Director',
    'TechVision Inc.',
    'BE1239876540',
    'S.A.',
    '789 Startup Road, Berlin, Germany',
    'https://www.techvision.com',
    '6203',
    300000.75,
    true,
    10,
    true,
    false,
    false,
    'No partner',
    'Application did not meet criteria.',
    'Disappointed but will try again.',
    false,
    false,
    true,
    'REJECTED'
);

INSERT INTO pfe.onboarding (
    firstname,
    lastname,
    email,
    password,
    position,
    company_name,
    company_number,
    legal_form,
    office_address,
    website,
    nace_code,
    revenue,
    franchise,
    nb_workers,
    dispute,
    honor_engagement,
    grant_application,
    grant_application_partner,
    something_else,
    comment,
    is_owner,
    offers_services,
    sells_products,
    status
) VALUES (
    'Charlie',
    'Brown',
    'charlie.brown@example.com',
    'password789',
    'Founder',
    'EcoFriendly Co.',
    'BE4567891230',
    'GmbH',
    '123 Sustainable Way, Amsterdam, Netherlands',
    'https://www.ecofriendly.com',
    '6204',
    1000000.00,
    false,
    15,
    false,
    true,
    true,
    'Funding Partners BV',
    'Application under review.',
    'Hopeful for approval.',
    true,
    true,
    true,
    'PENDING'
);

--questions
INSERT INTO pfe.questions_form(form_id, question_id,question_status)
VALUES
    (1,1,'COMPLETE'),
    (1,2,'COMPLETE'),
    (1,3,'COMPLETE'),
    (1,4,'COMPLETE'),
    (1,5,'COMPLETE'),
    (1,6,'COMPLETE'),
    (1,7,'COMPLETE'),
    (1,8,'COMPLETE'),
    (1,9,'COMPLETE'),
    (1,10,'COMPLETE'),
    (1,11,'COMPLETE'),
    (1,12,'COMPLETE'),
    (1,13,'COMPLETE'),
    (1,14,'COMPLETE'),
    (1,15,'COMPLETE'),
    (1,16,'COMPLETE'),
    (1,17,'COMPLETE'),
    (1,18,'COMPLETE'),
    (1,19,'COMPLETE'),
    (1,20,'COMPLETE'),
    (1,21,'COMPLETE'),
    (1,22,'COMPLETE'),
    (1,23,'COMPLETE'),
    (1,24,'COMPLETE'),
    (1,25,'COMPLETE'),
    (1,26,'COMPLETE'),
    (1,27,'COMPLETE'),
    (1,28,'COMPLETE'),
    (1,29,'COMPLETE'),
    (1,30,'COMPLETE'),
    (1,31,'COMPLETE'),
    (1,32,'COMPLETE'),
    (1,33,'COMPLETE'),
    (1,34,'COMPLETE'),
    (1,35,'COMPLETE'),
    (1,36,'COMPLETE'),
    (1,37,'COMPLETE'),
    (1,38,'COMPLETE'),
    (1,39,'COMPLETE'),
    (1,40,'COMPLETE'),
    (1,41,'COMPLETE'),
    (1,42,'COMPLETE'),
    (1,43,'COMPLETE'),
    (1,44,'COMPLETE'),
    (1,45,'COMPLETE'),
    (1,46,'COMPLETE'),
    (1,47,'COMPLETE'),
    (1,48,'COMPLETE'),
    (1,49,'COMPLETE'),
    (1,50,'COMPLETE'),
    (1,51,'COMPLETE'),
    (1,52,'COMPLETE'),
    (1,53,'COMPLETE'),
    (1,54,'COMPLETE'),
    (1,55,'COMPLETE'),
    (1,56,'COMPLETE'),
    (1,57,'COMPLETE'),
    (1,58,'COMPLETE'),
    (1,59,'COMPLETE'),
    (1,60,'COMPLETE'),
    (1,61,'COMPLETE'),
    (1,62,'COMPLETE'),
    (1,63,'COMPLETE'),
    (1,64,'COMPLETE'),
    (1,65,'COMPLETE'),
    (1,66,'COMPLETE'),
    (1,67,'COMPLETE'),
    (1,68,'COMPLETE'),
    (1,69,'COMPLETE'),
    (1,70,'COMPLETE'),
    (1,71,'COMPLETE'),
    (1,72,'COMPLETE'),
    (1,73,'COMPLETE'),
    (1,74,'COMPLETE'),
    (1,75,'COMPLETE'),
    (1,76,'COMPLETE'),
    (1,77,'COMPLETE'),
    (1,78,'COMPLETE'),
    (1,79,'COMPLETE'),
    (1,80,'COMPLETE'),
    (1,81,'COMPLETE'),
    (1,82,'COMPLETE'),
    (1,83,'COMPLETE'),
    (1,84,'COMPLETE'),
    (1,85,'PENDING'),
    (1,86,'PENDING'),
    (1,87,'PENDING'),
    (1,88,'PENDING'),
    (1,89,'PENDING'),
    (1,90,'PENDING'),
    (2,1,'COMPLETE'),
    (2,2,'COMPLETE'),
    (2,3,'COMPLETE'),
    (2,4,'COMPLETE'),
    (2,5,'COMPLETE'),
    (2,6,'COMPLETE'),
    (2,7,'COMPLETE'),
    (2,8,'COMPLETE'),
    (2,9,'COMPLETE'),
    (2,10,'COMPLETE'),
    (2,11,'COMPLETE'),
    (2,12,'COMPLETE'),
    (2,13,'COMPLETE'),
    (2,14,'COMPLETE'),
    (2,15,'COMPLETE'),
    (2,16,'COMPLETE'),
    (2,17,'COMPLETE'),
    (2,18,'COMPLETE'),
    (2,19,'COMPLETE'),
    (2,20,'COMPLETE'),
    (2,21,'COMPLETE'),
    (2,22,'COMPLETE'),
    (2,23,'COMPLETE'),
    (2,24,'COMPLETE'),
    (2,25,'COMPLETE'),
    (2,26,'COMPLETE'),
    (2,27,'COMPLETE'),
    (2,28,'COMPLETE'),
    (2,29,'COMPLETE'),
    (2,30,'COMPLETE'),
    (2,31,'COMPLETE'),
    (2,32,'COMPLETE'),
    (2,33,'COMPLETE'),
    (2,34,'COMPLETE'),
    (2,35,'COMPLETE'),
    (2,36,'COMPLETE'),
    (2,37,'COMPLETE'),
    (2,38,'COMPLETE'),
    (2,39,'COMPLETE'),
    (2,40,'COMPLETE'),
    (2,41,'COMPLETE'),
    (2,42,'COMPLETE'),
    (2,43,'COMPLETE'),
    (2,44,'COMPLETE'),
    (2,45,'COMPLETE'),
    (2,46,'COMPLETE'),
    (2,47,'COMPLETE'),
    (2,48,'COMPLETE'),
    (2,49,'COMPLETE'),
    (2,50,'COMPLETE'),
    (2,51,'COMPLETE'),
    (2,52,'COMPLETE'),
    (2,53,'COMPLETE'),
    (2,54,'COMPLETE'),
    (2,55,'COMPLETE'),
    (2,56,'COMPLETE'),
    (2,57,'COMPLETE'),
    (2,58,'COMPLETE'),
    (2,59,'COMPLETE'),
    (2,60,'COMPLETE'),
    (2,61,'COMPLETE'),
    (2,62,'COMPLETE'),
    (2,63,'COMPLETE'),
    (2,64,'COMPLETE'),
    (2,65,'COMPLETE'),
    (2,66,'COMPLETE'),
    (2,67,'COMPLETE'),
    (2,68,'COMPLETE'),
    (2,69,'COMPLETE'),
    (2,70,'COMPLETE'),
    (2,71,'COMPLETE'),
    (2,72,'COMPLETE'),
    (2,73,'COMPLETE'),
    (2,74,'COMPLETE'),
    (2,75,'COMPLETE'),
    (2,76,'COMPLETE'),
    (2,77,'COMPLETE'),
    (2,78,'COMPLETE'),
    (2,79,'COMPLETE'),
    (2,80,'COMPLETE'),
    (2,81,'COMPLETE'),
    (2,82,'COMPLETE'),
    (2,83,'COMPLETE'),
    (2,84,'COMPLETE'),
    (2,85,'COMPLETE'),
    (2,86,'COMPLETE'),
    (2,87,'COMPLETE'),
    (2,88,'COMPLETE'),
    (2,89,'COMPLETE'),
    (2,90,'COMPLETE'),
    (3,1,'COMPLETE'),
    (3,2,'COMPLETE'),
    (3,3,'COMPLETE'),
    (3,4,'COMPLETE'),
    (3,5,'COMPLETE'),
    (3,6,'COMPLETE'),
    (3,7,'COMPLETE'),
    (3,8,'COMPLETE'),
    (3,9,'COMPLETE'),
    (3,10,'COMPLETE'),
    (3,11,'COMPLETE'),
    (3,12,'COMPLETE'),
    (3,13,'COMPLETE'),
    (3,14,'COMPLETE'),
    (3,15,'COMPLETE'),
    (3,16,'COMPLETE'),
    (3,17,'COMPLETE'),
    (3,18,'COMPLETE'),
    (3,19,'COMPLETE'),
    (3,20,'COMPLETE'),
    (3,21,'COMPLETE'),
    (3,22,'COMPLETE'),
    (3,23,'COMPLETE'),
    (3,24,'COMPLETE'),
    (3,25,'COMPLETE'),
    (3,26,'COMPLETE'),
    (3,27,'COMPLETE'),
    (3,28,'COMPLETE'),
    (3,29,'COMPLETE'),
    (3,30,'COMPLETE'),
    (3,31,'COMPLETE'),
    (3,32,'COMPLETE'),
    (3,33,'COMPLETE'),
    (3,34,'COMPLETE'),
    (3,35,'COMPLETE'),
    (3,36,'COMPLETE'),
    (3,37,'COMPLETE'),
    (3,38,'COMPLETE'),
    (3,39,'COMPLETE'),
    (3,40,'COMPLETE'),
    (3,41,'COMPLETE'),
    (3,42,'COMPLETE'),
    (3,43,'COMPLETE'),
    (3,44,'COMPLETE'),
    (3,45,'COMPLETE'),
    (3,46,'COMPLETE'),
    (3,47,'COMPLETE'),
    (3,48,'COMPLETE'),
    (3,49,'COMPLETE'),
    (3,50,'COMPLETE'),
    (3,51,'COMPLETE'),
    (3,52,'COMPLETE'),
    (3,53,'COMPLETE'),
    (3,54,'COMPLETE'),
    (3,55,'COMPLETE'),
    (3,56,'COMPLETE'),
    (3,57,'COMPLETE'),
    (3,58,'COMPLETE'),
    (3,59,'COMPLETE'),
    (3,60,'COMPLETE'),
    (3,61,'COMPLETE'),
    (3,62,'COMPLETE'),
    (3,63,'COMPLETE'),
    (3,64,'COMPLETE'),
    (3,65,'COMPLETE'),
    (3,66,'COMPLETE'),
    (3,67,'COMPLETE'),
    (3,68,'COMPLETE'),
    (3,69,'COMPLETE'),
    (3,70,'COMPLETE'),
    (3,71,'COMPLETE'),
    (3,72,'COMPLETE'),
    (3,73,'COMPLETE'),
    (3,74,'COMPLETE'),
    (3,75,'COMPLETE'),
    (3,76,'COMPLETE'),
    (3,77,'COMPLETE'),
    (3,78,'COMPLETE'),
    (3,79,'COMPLETE'),
    (3,80,'COMPLETE'),
    (3,81,'COMPLETE'),
    (3,82,'COMPLETE'),
    (3,83,'COMPLETE'),
    (3,84,'COMPLETE'),
    (3,85,'COMPLETE'),
    (3,86,'COMPLETE'),
    (3,87,'COMPLETE'),
    (3,88,'COMPLETE'),
    (3,89,'COMPLETE'),
    (3,90,'COMPLETE');

--answers user
INSERT INTO pfe.user_answer_esg 
(answer_id,user_id,form_id,now,commitment_pact,comment,now_verif,commitment_pact_verif,status) 
VALUES 
    (1,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),
    
    (3,1,1,NULL,NULL,'test',NULL,NULL,'PENDING'),

    (4,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (8,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (12,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),
    (13,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),
    (14,1,1,FALSE,TRUE,NULL,NULL,NULL,'PENDING'),

    (20,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (24,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (26,1,1,NULL,NULL,'test',NULL,NULL,'PENDING'),

    (27,1,1,NULL,NULL,'test',NULL,NULL,'PENDING'),

    (28,1,1,NULL,NULL,'test',NULL,NULL,'PENDING'),

    (30,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (37,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),
    (38,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (43,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (50,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (54,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (59,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (64,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),
    (65,1,1,FALSE,TRUE,NULL,NULL,NULL,'PENDING'),
    (67,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),
    (68,1,1,FALSE,TRUE,NULL,NULL,NULL,'PENDING'),

    (72,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (75,1,1,NULL,NULL,'test',NULL,NULL,'PENDING'),

    (76,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),
    (77,1,1,FALSE,TRUE,NULL,NULL,NULL,'PENDING'),
    (81,1,1,NULL,NULL,'test',NULL,NULL,'PENDING'),

    (84,1,1,FALSE,TRUE,NULL,NULL,NULL,'PENDING'),

    (88,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),
    (89,1,1,FALSE,TRUE,NULL,NULL,NULL,'PENDING'),
    (90,1,1,NULL,NULL,'test',NULL,NULL,'PENDING'),

    (93,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (100,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (106,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (109,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (112,1,1,NULL,NULL,'test',NULL,NULL,'PENDING'),

    (113,1,1,NULL,NULL,'test',NULL,NULL,'PENDING'),

    (114,1,1,FALSE,TRUE,NULL,NULL,NULL,'PENDING'),
    (115,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),
    (116,1,1,FALSE,TRUE,NULL,NULL,NULL,'PENDING'),
    (117,1,1,FALSE,TRUE,NULL,NULL,NULL,'PENDING'),
    (118,1,1,FALSE,TRUE,NULL,NULL,NULL,'PENDING'),
    (119,1,1,FALSE,TRUE,NULL,NULL,NULL,'PENDING'),
    (120,1,1,FALSE,TRUE,NULL,NULL,NULL,'PENDING'),
    (121,1,1,TRUE,TRUE,NULL,NULL,NULL,'PENDING'),

    (125,1,1,TRUE,TRUE,NULL,NULL,NULL,'PENDING'),

    (128,1,1,FALSE,TRUE,NULL,NULL,NULL,'PENDING'),
    (129,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),
    (131,1,1,FALSE,TRUE,NULL,NULL,NULL,'PENDING'),

    (136,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),
    (137,1,1,FALSE,TRUE,NULL,NULL,NULL,'PENDING'),
    (139,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),
    (140,1,1,FALSE,TRUE,NULL,NULL,NULL,'PENDING'),

    (145,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (148,1,1,TRUE,TRUE,NULL,NULL,NULL,'PENDING'),
    (149,1,1,FALSE,TRUE,NULL,NULL,NULL,'PENDING'),
    (151,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (155,1,1,FALSE,TRUE,NULL,NULL,NULL,'PENDING'),
    (157,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (161,1,1,FALSE,TRUE,NULL,NULL,NULL,'PENDING'),
    (162,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),
    (163,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (167,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (170,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (173,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (179,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (187,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (193,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (198,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (204,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (210,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (219,1,1, NULL,NULL,'test',NULL,NULL,'PENDING'),

    (222,1,1,TRUE,TRUE,NULL,NULL,NULL,'PENDING'),

    (229,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (232,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (235,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (244,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (250,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (256,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (262,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (268,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (274,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (280,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (286,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (292,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (298,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (306,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (314,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (320,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (326,1,1,FALSE,TRUE,NULL,NULL,NULL,'PENDING'),

    (328,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (333,1,1,NULL,NULL,'test',NULL,NULL,'PENDING'),

    (334,1,1,NULL,NULL,'test',NULL,NULL,'PENDING'),

    (335,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (341,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (343,1,1,NULL,NULL,'test',NULL,NULL,'PENDING'),

    (345,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (351,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (357,1,1,TRUE,TRUE,NULL,NULL,NULL,'PENDING'),

    (367,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (374,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (378,1,1,FALSE,TRUE,NULL,NULL,NULL,'PENDING'),
    (380,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),
    
    (385,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (395,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (406,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (414,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (423,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),
    (425,1,1,FALSE,TRUE,NULL,NULL,NULL,'PENDING'),

    (431,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (441,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (448,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING'),

    (454,1,1,TRUE,FALSE,NULL,NULL,NULL,'PENDING');

    INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (1, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (3, 2, 2, null, null, 'test', null, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (4, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (8, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (12, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (13, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (14, 2, 2, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (20, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (24, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (26, 2, 2, null, null, 'test', null, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (27, 2, 2, null, null, 'test', null, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (28, 2, 2, null, null, 'test', null, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (30, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (37, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (38, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (43, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (50, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (54, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (59, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (64, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (65, 2, 2, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (67, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (68, 2, 2, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (72, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (75, 2, 2, null, null, 'test', null, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (76, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (77, 2, 2, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (81, 2, 2, null, null, 'test', null, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (84, 2, 2, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (88, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (89, 2, 2, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (90, 2, 2, null, null, 'test', null, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (93, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (100, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (106, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (109, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (112, 2, 2, null, null, 'test', null, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (113, 2, 2, null, null, 'test', null, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (114, 2, 2, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (115, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (116, 2, 2, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (117, 2, 2, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (118, 2, 2, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (119, 2, 2, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (120, 2, 2, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (121, 2, 2, true, true, null, true, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (125, 2, 2, true, true, null, true, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (128, 2, 2, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (129, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (131, 2, 2, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (136, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (137, 2, 2, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (139, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (140, 2, 2, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (145, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (148, 2, 2, true, true, null, true, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (149, 2, 2, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (151, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (155, 2, 2, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (157, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (161, 2, 2, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (162, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (163, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (167, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (170, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (173, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (179, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (187, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (193, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (198, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (204, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (210, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (219, 2, 2, null, null, 'test', null, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (222, 2, 2, true, true, null, true, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (229, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (232, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (235, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (244, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (250, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (256, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (262, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (268, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (274, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (280, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (286, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (292, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (298, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (306, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (314, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (320, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (326, 2, 2, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (328, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (333, 2, 2, null, null, 'test', null, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (334, 2, 2, null, null, 'test', null, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (335, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (341, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (343, 2, 2, null, null, 'test', null, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (345, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (351, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (357, 2, 2, true, true, null, true, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (367, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (374, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (378, 2, 2, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (380, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (385, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (395, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (406, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (414, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (423, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (425, 2, 2, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (431, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (441, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (448, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (454, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (456, 2, 2, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (458, 2, 2, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (464, 2, 2, true, false, null, null, null, 'PENDING');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (468, 2, 2, false, true, null, null, null, 'PENDING');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (470, 2, 2, true, false, null, null, null, 'PENDING');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (472, 2, 2, true, false, null, null, null, 'PENDING');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (485, 2, 2, true, false, null, null, null, 'PENDING');

INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (1, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (3, 3, 3, null, null, 'test', null, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (4, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (8, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (12, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (13, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (14, 3, 3, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (20, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (24, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (26, 3, 3, null, null, 'test', null, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (27, 3, 3, null, null, 'test', null, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (28, 3, 3, null, null, 'test', null, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (30, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (37, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (38, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (43, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (50, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (54, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (59, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (64, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (65, 3, 3, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (67, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (68, 3, 3, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (72, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (75, 3, 3, null, null, 'test', null, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (76, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (77, 3, 3, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (81, 3, 3, null, null, 'test', null, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (84, 3, 3, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (88, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (89, 3, 3, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (90, 3, 3, null, null, 'test', null, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (93, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (100, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (106, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (109, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (112, 3, 3, null, null, 'test', null, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (113, 3, 3, null, null, 'test', null, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (114, 3, 3, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (115, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (116, 3, 3, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (117, 3, 3, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (118, 3, 3, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (119, 3, 3, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (120, 3, 3, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (121, 3, 3, true, true, null, true, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (125, 3, 3, true, true, null, true, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (128, 3, 3, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (129, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (131, 3, 3, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (136, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (137, 3, 3, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (139, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (140, 3, 3, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (145, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (148, 3, 3, true, true, null, true, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (149, 3, 3, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (151, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (155, 3, 3, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (157, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (161, 3, 3, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (162, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (163, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (167, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (170, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (173, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (179, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (187, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (193, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (198, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (204, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (210, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (219, 3, 3, null, null, 'test', null, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (222, 3, 3, true, true, null, true, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (229, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (232, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (235, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (244, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (250, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (256, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (262, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (268, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (274, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (280, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (286, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (292, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (298, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (306, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (314, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (320, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (326, 3, 3, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (328, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (333, 3, 3, null, null, 'test', null, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (334, 3, 3, null, null, 'test', null, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (335, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (341, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (343, 3, 3, null, null, 'test', null, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (345, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (351, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (357, 3, 3, true, true, null, true, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (367, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (374, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (378, 3, 3, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (380, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (385, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (395, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (406, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (414, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (423, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (425, 3, 3, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (431, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (441, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (448, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (454, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (456, 3, 3, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (458, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (464, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (468, 3, 3, false, true, null, null, true, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (470, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (472, 3, 3, true, false, null, true, null, 'VALIDATED');
INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status) VALUES (485, 3, 3, true, false, null, true, null, 'VALIDATED');


