![Build Status](https://github.com/julien-remmery-vinci/bloc3-PFE-backend/actions/workflows/rust.yml/badge.svg)
# Projet de fin d'études - Backend

Bienvenue dans le repository de l'equipe backend de ce projet de fin d'études.

Le but de ce projet était de créer une application de gestion de questionnaires pour une entreprise afin de rendre leur travail moins administratif. 

Notre application permet la gestion de la prise de contact entre notre client et ses clients ainsi que leur acceptation ou rejet en fonction de leur éligibilité. Suite à cette étape un client pourra compléter un questionnaire qui sera ensuite envoyé pour review à un administrateur de l'entreprise, un score sera attribué au questionnaire après validation des réponses. Les administrateurs de l'entreprise ont accés à des statistiques sur certaines informations concernant l'application et ses utilisateurs.

Pour réaliser cette application nous avons opté pour une API Rest en Rust, un frontend en Angular ainsi qu'une base de données PostgreSQL.

Le code de la partie [frontend](https://github.com/julien-remmery-vinci/bloc3-PFE-frontend) se trouve dans un repository séparé.

## Ce repository contient :
    - Le code source de notre application
    - Les fichiers nécéssaires pour peupler la base de données
    - Les fichiers docker nécéssaire pour déployer l'application dans un container. (Le fichier compose contient aussi le container frontend, dont le code source est disponnible dans un autre repository)
    - Des requetes http utilisées pour tester l'application.
    - Les spécifications de notre api selon le standard OpenAPI

## Installation & lancement :

### Instalation préalable :
1. Installation de [Rust et de ses outils](https://www.rust-lang.org/fr/tools/install)
1. Cloner le repository
2. Se déplacer dans le dossier cloné

### Lancer le projet :
1. Avoir une base de donnée PostgreSQL à disposition
2. Renommer le fichier .env.example en .env et y ajouter ce qui suit :
    - DATABASE_URL : lien de connection à votre base de données
    - JWT_SECRET : secret jwt utilisé pour le chiffrement des token d'authentification
    - HASH_ROUNDS : nombre de 'rounds' nécéssaires au chiffrement des mots de passe avant stockage en base de données
3. Build le projet
```console
cargo build
``` 
4. Lancer le projet
```console
cargo run
``` 

## Utilisation du projet :
Afin d'utiliser au mieux notre projet il est recommandé de lancer le frontend en même temps, ceci peut être fait en suivant les instructions présentes dans le README de cette partie du projet.

Il est toute fois possible d'utiliser les requetes présentes dans le dossier requests pour tester l'application sans le frontend.

## Auteurs de la partie backend :
- Hadjera Emroska
- Hajar Haouat
- Julien Remmery
- Semih Turkoglu

En collaboration avec l'équipe frontend.

## Liens utiles :

- [Documentation](https://doc.rust-lang.org/book/) du langage Rust
- [Rustlings](https://github.com/rust-lang/rustlings) : Exercices d'apprentissage du langage Rust
- [Rust analyzer](https://code.visualstudio.com/docs/languages/rust) : extension VS Code, (coloration syntaxique, auto complétion, ...)