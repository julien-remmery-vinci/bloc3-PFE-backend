INSERT INTO pfe.users (login, password)
VALUES 

('admin@example.com', '$2a$10$2PYC2hW.wb9q5mf.xpL6IOi3C03eH3OKYZYOtqtGNMFAJBeI6YLWe'), 
('user1@example.com', '$2a$10$2PYC2hW.wb9q5mf.xpL6IOi3C03eH3OKYZYOtqtGNMFAJBeI6YLWe');
INSERT INTO pfe.questions (question_status, category, sub_category, question)
VALUES
-- 1. ENERGIE & CARBONE
('A compléter', '1. ENERGIE & CARBONE', '1.1. GESTION DE L''ENERGIE', 'Suivez-vous la consommation d''énergie de XXX ?'),
('A compléter', '1. ENERGIE & CARBONE', '1.1. GESTION DE L''ENERGIE', 'Si vous la suivez, veuillez indiquer votre consommation d''énergie totale des 12 derniers mois (en kWh)'),
('A compléter', '1. ENERGIE & CARBONE', '1.1. GESTION DE L''ENERGIE', 'Avez-vous un contrat vert avec votre fournisseur d''énergie ?'),
('A compléter', '1. ENERGIE & CARBONE', '1.1. GESTION DE L''ENERGIE', 'Produisez-vous de l''électricité sur site (par exemple, avec des panneaux solaires) ?'),
('A compléter', '1. ENERGIE & CARBONE', '1.1. GESTION DE L''ENERGIE', 'Qu''avez-vous mis en place pour limiter votre consommation d''énergie ? Veuillez sélectionner toutes les réponses applicables.'),
('A compléter', '1. ENERGIE & CARBONE', '1.1. GESTION DE L''ENERGIE', 'Avez-vous mis en place des initiatives pour sensibiliser vos travailleur·euses sur les bonnes pratiques en matière de consommation d''énergie ?'),
('A compléter', '1. ENERGIE & CARBONE', '1.2. EMPREINTE CARBONE', 'Avez-vous calculé l''empreinte carbone des activités de XXX ?'),
('A compléter', '1. ENERGIE & CARBONE', '1.2. EMPREINTE CARBONE', 'Si vous la calculez, veuillez indiquer votre empreinte carbone de Scope 1 (en tonnes de CO2 équivalent) :'),
('A compléter', '1. ENERGIE & CARBONE', '1.2. EMPREINTE CARBONE', 'Si vous la calculez, veuillez indiquer votre empreinte carbone de Scope 2 (en tonnes de CO2 équivalent) :'),
('A compléter', '1. ENERGIE & CARBONE', '1.2. EMPREINTE CARBONE', 'Si vous la calculez, veuillez indiquer votre empreinte carbone de Scope 3 (en tonnes de CO2 équivalent) :'),
('A compléter', '1. ENERGIE & CARBONE', '1.2. EMPREINTE CARBONE', 'Si vous avez réalisé un rapport d''empreinte carbone pour XXX, que comprend-il ? Veuillez sélectionner toutes les réponses applicables.'),
('A compléter', '1. ENERGIE & CARBONE', '1.2. EMPREINTE CARBONE', 'Quelle(s) pratique(s) avez-vous mise(s) en place pour limiter votre empreinte carbone ?'),
('A compléter', '1. ENERGIE & CARBONE', '1.2. EMPREINTE CARBONE', 'Proposez-vous des ateliers de sensibilisation et des formations sur les émissions de gaz à effet de serre à destination de vos travailleur·euses ? (par exemple, la fresque du climat)'),
('A compléter', '1. ENERGIE & CARBONE', '1.2. EMPREINTE CARBONE', 'Quel pourcentage de vos fournisseurs (en valeur monétaire) se trouvent à moins de 80 km de vos installations ?'),
('A compléter', '1. ENERGIE & CARBONE', '1.2. EMPREINTE CARBONE', 'Quel pourcentage de votre transport amont et aval (approvisionnement et livraison) est effectué à l''aide de modes de transport à faibles émissions de carbone (par exemple, vélo-cargo, train, voilier, etc.) ?'),
('A compléter', '1. ENERGIE & CARBONE', '1.2. EMPREINTE CARBONE', 'Avez-vous optimisé votre chaîne de valeur afin de réduire les distances de transport, par exemple en rapprochant vos fournisseurs, en centralisant vos sites de production ou en réorganisant vos réseaux de distribution ?'),
('A compléter', '1. ENERGIE & CARBONE', '1.2. EMPREINTE CARBONE', 'Quelle(s) mesure(s) avez-vous prise(s) avec vos fournisseurs pour limiter les émissions carbones liées à votre chaine d''approvisionnement ?'),

-- 2. EAU, MATIERES PREMIERES ET FOURNITURES
('A compléter', '2. EAU, MATIERES PREMIERES ET FOURNITURES', '2.1. EAU', 'Mesurez-vous la consommation d''eau de XXX ?'),
('A compléter', '2. EAU, MATIERES PREMIERES ET FOURNITURES', '2.1. EAU', 'Si vous la mesurez, veuillez indiquer votre consommation d''eau (en litres) :'),
('A compléter', '2. EAU, MATIERES PREMIERES ET FOURNITURES', '2.1. EAU', 'Qu''avez-vous mis en place pour limiter votre consommation d''eau ? Veuillez sélectionner toutes les réponses applicables.'),
('A compléter', '2. EAU, MATIERES PREMIERES ET FOURNITURES', '2.1. EAU', 'Avez-vous mis en place des initiatives pour sensibiliser vos travailleur·euses à la gestion de l''eau et à sa consommation ?'),
('A compléter', '2. EAU, MATIERES PREMIERES ET FOURNITURES', '2.1. EAU', 'XXX a-t-elle pris l''une ou plusieurs de ces mesures concernant la consommation d''eau dans sa chaine d''approvisionnement ?'),
('A compléter', '2. EAU, MATIERES PREMIERES ET FOURNITURES', '2.2. MATIERES PREMIERES ET FOURNITURES', 'XXX a-t-elle pris l''une ou plusieurs de ces mesures concernant l''utilisation des ressources ?'),
('A compléter', '2. EAU, MATIERES PREMIERES ET FOURNITURES', '2.2. MATIERES PREMIERES ET FOURNITURES', 'Avez-vous mis en place des initiatives pour sensibiliser vos travailleur·euses à l''utilisation des ressources ?'),
('A compléter', '2. EAU, MATIERES PREMIERES ET FOURNITURES', '2.2. MATIERES PREMIERES ET FOURNITURES', 'Quel pourcentage de vos matériaux (en volume) provient de matériaux recyclés, de composants réutilisés et/ou de matériaux certifiés d''origine durable ?'),

-- 3. DÉCHETS
('A compléter', '3. DÉCHETS', '3.1. DÉCHETS', 'Suivez-vous les quantités et les types de déchets produits par XXX ?'),
('A compléter', '3. DÉCHETS', '3.1. DÉCHETS', 'Si vous la suivez, veuillez indiquer la quantité totale de déchets produits par votre organisation au cours des 12 derniers mois (en tonnes) ?'),
('A compléter', '3. DÉCHETS', '3.1. DÉCHETS', 'Si vous la suivez, veuillez indiquer la quantité de déchets dangereux produits par votre organisation au cours des 12 derniers mois (en tonnes) ?
(exemples de déchets dangereux : équipements électroniques, piles, cartouches d''encre, déchets industriels, batteries, peintures, etc.)'),
('A compléter', '3. DÉCHETS', '3.1. DÉCHETS', 'Quelle(s) pratique(s) avez-vous mise(s) en place pour réduire la production de déchets et optimiser leur gestion ?'),
('A compléter', '3. DÉCHETS', '3.1. DÉCHETS', 'Avez-vous mis en place des initiatives pour sensibiliser vos travailleur·euses à la réduction, la gestion et le recyclage des déchets ?'),
('A compléter', '3. DÉCHETS', '3.1. DÉCHETS', 'Quelle(s) pratique(s) XXX a-t-elle mise(s) en place en matière d''économie circulaire pour limiter la production de déchets ?'),

-- 4. ÉCOSYSTÈMES ET BIODIVERSITÉ
('A compléter', '4. ÉCOSYSTÈMES ET BIODIVERSITÉ', '4.1. ÉCOSYSTÈMES ET BIODIVERSITÉ', 'XXX a-t-elle mis en place une ou plusieurs initiatives visant à protéger les écosystèmes et préserver la biodiversité ?'),
('A compléter', '4. ÉCOSYSTÈMES ET BIODIVERSITÉ', '4.1. ÉCOSYSTÈMES ET BIODIVERSITÉ', 'Proposez-vous des ateliers de sensibilisation et des formations sur biodiversité à destination de vos travailleur·euses ?'),

-- 5. DIVERSITE, INCLUSION ET EQUITE
('A compléter', '5. DIVERSITE, INCLUSION ET EQUITE', '5.1. INCLUSION ET EQUITE', 'Comment XXX crée-t-elle un processus de recrutement et d''embauche inclusif ?'),
('A compléter', '5. DIVERSITE, INCLUSION ET EQUITE', '5.1. INCLUSION ET EQUITE', 'Comment XXX crée-t-elle un lieu de travail équitable et inclusif pour les travailleur·euses ?'),
('A compléter', '5. DIVERSITE, INCLUSION ET EQUITE', '5.1. INCLUSION ET EQUITE', 'En dehors du règlement de travail légal, quelles sont la ou les pratiques suivantes que XXX a mis en oeuvre en matière de diversité et d''inclusion ?'),
('A compléter', '5. DIVERSITE, INCLUSION ET EQUITE', '5.1. INCLUSION ET EQUITE', 'Est-ce que XXX monitore les formations en matière de diversité et d''inclusion ?'),
('A compléter', '5. DIVERSITE, INCLUSION ET EQUITE', '5.1. INCLUSION ET EQUITE', 'La politique salariale de XXX est-elle régulièrement examinée pour s''assurer que les travailleur·euses bénéficient d''un traitement égal pour un travail égal ?'),
('A compléter', '5. DIVERSITE, INCLUSION ET EQUITE', '5.1. INCLUSION ET EQUITE', 'Quel est le multiple du salaire de la personne la mieux rémunérée, y compris les primes, par rapport à celui de la personne à temps plein la moins bien rémunérée ?'),
('A compléter', '5. DIVERSITE, INCLUSION ET EQUITE', '5.2. DIVERSITE', 'Quel pourcentage des travailleur·euses est âgé de moins de vingt-quatre ans ou de plus de cinquante ans ?'),
('A compléter', '5. DIVERSITE, INCLUSION ET EQUITE', '5.2. DIVERSITE', 'Quel est le pourcentage des cadres de XXX qui s''identifient comme femmes ?'),
('A compléter', '5. DIVERSITE, INCLUSION ET EQUITE', '5.2. DIVERSITE', 'Quel est le pourcentage des travailleur·euses de XXX qui s''identifient comme femmes ?'),
('A compléter', '5. DIVERSITE, INCLUSION ET EQUITE', '5.2. DIVERSITE', 'Quel pourcentage des travailleur·euses se considère comme appartenant à un groupe social sous-représenté (ex : personnes LGBTQIA+, issues de minorités ethniques, etc.)'),
('A compléter', '5. DIVERSITE, INCLUSION ET EQUITE', '5.2. DIVERSITE', 'Quel pourcentage des travailleur·euses de XXX fait partie d''un groupe souvent exclu du marché du travail (ex : personnes en situation de handicap, anciens détenus, jeunes demandeurs d''emploi, chômeurs de longue durée, etc.) ?'),

-- 6. SÉCURITÉ, SANTÉ & BIEN-ÊTRE
('A compléter', '6. SÉCURITÉ, SANTÉ & BIEN-ÊTRE', '6.1. SECURITE AU TRAVAIL', 'Comment XXX démontre-t-elle son engagement à garantir la santé et la sécurité des travailleur·euses, en allant au-delà des obligations légales ?'),
('A compléter', '6. SÉCURITÉ, SANTÉ & BIEN-ÊTRE', '6.1. SECURITE AU TRAVAIL', 'Est-ce que XXX mesure le taux d''accidents dans l''entreprise par année fiscale ? Si oui, veuillez préciser ce taux : Nombre d''accidents du travail au cours de l''année de référence/ Nombre total d''heures travaillées au cours d''une année par l''ensemble des salariés)*200 000. Indication : 1710 heures travaillées en 2023 pour un équivalent temps plein en Belgique (225*7,6)'),
('A compléter', '6. SÉCURITÉ, SANTÉ & BIEN-ÊTRE', '6.2. SANTÉ ET BIEN-ÊTRE', 'Quelles initiatives ou politiques en matière de santé et de bien-être XXX propose-t-elle à ses travailleur·euses ?'),
('A compléter', '6. SÉCURITÉ, SANTÉ & BIEN-ÊTRE', '6.2. SANTÉ ET BIEN-ÊTRE', 'Est-ce que XXX mesure le taux d''absentéisme dans l''entreprise par année fiscale ? Si oui, veuillez préciser ce taux : nombre de jours d''absentéisme/Nombre de jours de travail * nombre de salariés * 200 000'),

-- 7. EMPLOI ET PRATIQUES DE TRAVAIL
('A compléter', '7. EMPLOI ET PRATIQUES DE TRAVAIL', '7.1. DEVELOPPEMENT DES COMPETENCES', 'Est-ce que XXX a mis en place une politique de formation des travailleur·euses qui prévoit un certain nombre d''heures de formation et/ou un budget annuel par travailleur·euse ?'),
('A compléter', '7. EMPLOI ET PRATIQUES DE TRAVAIL', '7.1. DEVELOPPEMENT DES COMPETENCES', 'Est-ce que XXX offre aux travailleur·euses l''une des possibilités de formation suivantes en vue de leur développement professionnel ?'),
('A compléter', '7. EMPLOI ET PRATIQUES DE TRAVAIL', '7.1. DEVELOPPEMENT DES COMPETENCES', 'Est-ce que XXX mesure le pourcentage de travailleur·euses ayant bénéficié d''une formation au cours des 12 derniers mois ? Si oui, veuillez préciser ce pourcentage.'),
('A compléter', '7. EMPLOI ET PRATIQUES DE TRAVAIL', '7.1. DEVELOPPEMENT DES COMPETENCES', 'Quel est le montant total des dépenses consacrées à la formation des travailleur·euses au cours de la dernière année fiscale ?'),
('A compléter', '7. EMPLOI ET PRATIQUES DE TRAVAIL', '7.1. DEVELOPPEMENT DES COMPETENCES', 'Quel est le pourcentage de travailleur·euses ayant bénéficié d''une promotion interne au cours des 12 derniers mois ?'),
('A compléter', '7. EMPLOI ET PRATIQUES DE TRAVAIL', '7.1. DEVELOPPEMENT DES COMPETENCES', 'Comment est-ce que XXX engage et responsabilise les travailleur·euses ?'),
('A compléter', '7. EMPLOI ET PRATIQUES DE TRAVAIL', '7.2. ENGAGEMENT ET SATISFACTION', 'Lesquelles des affirmations suivantes sont vraies en ce qui concerne la satisfaction et/ou l''engagement des travailleur·euses au sein de XXX ?'),
('A compléter', '7. EMPLOI ET PRATIQUES DE TRAVAIL', '7.2. ENGAGEMENT ET SATISFACTION', 'Quel est le pourcentage équivalent du chiffre d''affaires distribué aux travailleur·euses sous forme de prime au cours du dernier exercice fiscal ?'),
('A compléter', '7. EMPLOI ET PRATIQUES DE TRAVAIL', '7.2. ENGAGEMENT ET SATISFACTION', 'Quel est le pourcentage de travailleur·euses ayant bénéficié d''une prime au cours du dernier exercice fiscal ?'),
('A compléter', '7. EMPLOI ET PRATIQUES DE TRAVAIL', '7.2. ENGAGEMENT ET SATISFACTION', 'Quel est le pourcentage de vos travailleur·euses qui sont "Satisfait·es" ou "Engagé·es" ?'),
('A compléter', '7. EMPLOI ET PRATIQUES DE TRAVAIL', '7.2. ENGAGEMENT ET SATISFACTION', 'Quel est le turn-over en pourcentage de XXX ? (Sur l''année fiscale précédente, diviser le nombre de départs d''employés (volontaires ou non) par l''effectif moyen de l''entreprise, puis multiplier le résultat par 100 pour obtenir un pourcentage)'),

-- 8. ENGAGEMENT CIVIQUE
('A compléter', '8. ENGAGEMENT CIVIQUE', '8.1. ENGAGEMENT SOCIAL', 'Comment XXX s''implique-t-elle dans des projets sociaux ?'),
('A compléter', '8. ENGAGEMENT CIVIQUE', '8.1. ENGAGEMENT SOCIAL', 'Est-ce que XXX monitore régulièrement les heures consacrées à l''engagement social ?'),
('A compléter', '8. ENGAGEMENT CIVIQUE', '8.2. PHILANTHROPIE', 'Comment XXX aborde-t-elle la philanthropie ?'),
('A compléter', '8. ENGAGEMENT CIVIQUE', '8.2. PHILANTHROPIE', 'Quel est le pourcentage du chiffre d''affaires distribué à titre philanthropique au cours du dernier exercice fiscal ?'),

-- 9. CONDUITE DES AFFAIRES
('A compléter', '9. CONDUITE DES AFFAIRES', '9.1. STRUCTURE DE GOUVERNANCE', 'Laquelle de ces options décrit le mieux la structure de gouvernance de XXX ?'),
('A compléter', '9. CONDUITE DES AFFAIRES', '9.1. STRUCTURE DE GOUVERNANCE', 'Le conseil d''administration de XXX dispose-t-il de sièges avec droit de vote représentant :'),
('A compléter', '9. CONDUITE DES AFFAIRES', '9.1. STRUCTURE DE GOUVERNANCE', 'Combien de membres compte votre conseil d''administration au total ?'),
('A compléter', '9. CONDUITE DES AFFAIRES', '9.1. STRUCTURE DE GOUVERNANCE', 'Quel est le pourcentage d''administateur·rices indépendant·es au sein de votre conseil d''administration ?'),
('A compléter', '9. CONDUITE DES AFFAIRES', '9.1. STRUCTURE DE GOUVERNANCE', 'Quel est le pourcentage des membres de votre conseil d''administration qui s''identifient comme femmes ?'),
('A compléter', '9. CONDUITE DES AFFAIRES', '9.1. STRUCTURE DE GOUVERNANCE', 'Tous·tes les membres du conseil d''administration et de la direction remplissent-ils·elles un questionnaire annuel sur les conflits d''intérêts ?'),
('A compléter', '9. CONDUITE DES AFFAIRES', '9.1. STRUCTURE DE GOUVERNANCE', 'Combien de réunions du conseil d''administration avez-vous tenues au cours des 12 derniers mois ?'),
('A compléter', '9. CONDUITE DES AFFAIRES', '9.2. INTEGRATION DES PARTIES PRENANTES', 'Quel pourcentage des principaux fournisseurs fait l''objet d''audits ou contrôles annuels d''assurance qualité ?'),
('A compléter', '9. CONDUITE DES AFFAIRES', '9.2. INTEGRATION DES PARTIES PRENANTES', 'Parmi les initiatives suivantes, quelles sont celles qui ont été mises en oeuvre par XXX pour atténuer les risques ESG au sein de sa chaîne d''approvisionnement ?'),
('A compléter', '9. CONDUITE DES AFFAIRES', '9.2. INTEGRATION DES PARTIES PRENANTES', 'XXX met-elle en place une ou plusieurs des actions suivantes pour gérer la valeur qu''elle apporte à ses client·es ou consommateur·rices ?'),
('A compléter', '9. CONDUITE DES AFFAIRES', '9.2. INTEGRATION DES PARTIES PRENANTES', 'Comment XXX intègre-t-elle les performances sociales et environnementales dans son processus décisionnel ?'),
('A compléter', '9. CONDUITE DES AFFAIRES', '9.3. GESTION DURABLE', 'Avez-vous une mission formelle et écrite qui comprend l''un des élements suivants ? 
Une déclaration de mission formelle et écrite de l''entreprise est une déclaration publique ou formellement partagée avec les travailleur·euses de l''entreprise.'),
('A compléter', '9. CONDUITE DES AFFAIRES', '9.3. GESTION DURABLE', 'Comment XXX intègre-t-elle les Objectifs de Développement Durables (ODD) dans sa stratégie globale et ses activités commerciales ?'),
('A compléter', '9. CONDUITE DES AFFAIRES', '9.3. GESTION DURABLE', 'Quelles sont les étapes suivantes de l''analyse de double matérialité de XXX que vous avez déjà mises en oeuvres ?'),
('A compléter', '9. CONDUITE DES AFFAIRES', '9.4. TRANSPARENCE', 'Quelles informations sur son impact environnemental XXX rend-elle accessibles et transparentes ?'),
('A compléter', '9. CONDUITE DES AFFAIRES', '9.4. TRANSPARENCE', 'Quelles informations sur son impact social XXX rend-elle accessibles et transparentes ?'),
('A compléter', '9. CONDUITE DES AFFAIRES', '9.4. TRANSPARENCE', 'Quelles informations sur sa gouvernance XXX rend-elle accessibles et transparentes ?'),
('A compléter', '9. CONDUITE DES AFFAIRES', '9.4. TRANSPARENCE', 'Quelles actions XXX a-t-elle mises en place pour garantir la qualité de son rapport d''impact ?'),

-- 10. ETHIQUE DES AFFAIRES
('A compléter', '10. ETHIQUE DES AFFAIRES', '10.1. FORMALISATION DES PRATIQUES', 'Quelle est l''approche de XXX en matière de formalisation de ses engagements : avez-vous écrit et partagé en interne les politiques suivantes ?'),
('A compléter', '10. ETHIQUE DES AFFAIRES', '10.2. LUTTE CONTRE LA CORRUPTION FINANCIÈRE', 'Le programme de lutte contre la corruption de XXX inclut-il un ou plusieurs des éléments suivants ?'),
('A compléter', '10. ETHIQUE DES AFFAIRES', '10.3. DIVULGATION DES LITIGES', 'XXX a-t-elle été confrontée à un litige important au cours de l''année écoulée (Si oui, veuillez préciser) ?'),
('A compléter', '10. ETHIQUE DES AFFAIRES', '10.3. DIVULGATION DES LITIGES', 'XXX a-t-elle été confrontée à un problème environnemental majeur ou à un litige au cours de l''année écoulée. (Si oui, veuillez préciser) ?'),

-- 11. PROTECTION DES DONNEES
('A compléter', '11. PROTECTION DES DONNEES', '11.1. SECURITE DES DONNEES', 'Parmi les initiatives suivantes, laquelle ou lesquelles ont été mises en œuvre pour assurer la protection des données ?'),
('A compléter', '11. PROTECTION DES DONNEES', '11.2. PROTECTION DE LA VIE PRIVEE', 'Laquelle de ces propositions décrit le mieux l''état de votre politique de protection de la vie privée des employé·es et client·es ?'),
('A compléter', '11. PROTECTION DES DONNEES', '11.2. PROTECTION DE LA VIE PRIVEE', 'XXX respecte-t-elle toutes les obligations spécifiques du RGPD en ce qui concerne la collecte, l''utilisation et la protection des données  acquises par le biais de son site web. ?'),

-- 12. CERTIFICATIONS
('A compléter', '12. CERTIFICATIONS', '12.1. CERTIFICATIONS D''UN PRODUIT, D''UN SERVICE OU D''UNE PRATIQUE', 'Quel est le pourcentage de vos produits ou services ou pratiques durables qui ont été contrôlés et certifiés par un organisme d''accréditation  axé sur la qualité ou le développement durable ? (ex : Fairtrade, BIO, Ecovadis, Ethibel, BDO etc) '),
('A compléter', '12. CERTIFICATIONS', '12.2. CERTIFICATIONS DE L''ENTREPRISE', 'XXX a-t-elle obtenu une ou plusieurs certifications attestant ses efforts en matière de transition durable ?');

INSERT INTO pfe.answers (answer, template, question_id, score, engagement_score, is_forced_engagement, comment)
VALUES
('oui','ALL',1,2,0.5,FALSE,'')