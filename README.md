# Spécification - Simulation Estajoj

## État d'implémentation
✅ Structure de base (estajo.rs, event.rs)
✅ Tests unitaires de base
✅ Méthodes d'interaction
✅ Gestion des besoins
✅ Système d'historisation
✅ Paramètres de simulation
✅ Interface utilisateur
✅ Persistance des données

## 1. Définitions

### 1.1 Estajo
- Être virtuel (du mot Esperanto "estajo" signifiant "un être")
- Singulier: estajo
- Pluriel: estajoj

### 1.2 Caractéristiques d'un Estajo
- Identifiant unique
- Nom
- Sexe (Male/Female)
- Points de vie
- Besoins fondamentaux:
  - Nourriture (0-100%)
  - Ambition (0-100%)
- Génétique :
  - Facteur d'énergie
  - Facteur de satisfaction
  - Facteur d'influence
- Historique des événements

## 2. Interactions
### 2.1 Actions possibles
- Aider: impact positif sur la cible
- Blesser: diminue les points de vie
- Comploter: influence les relations

### 2.2 Besoins identifiés
- Se nourrir: nécessaire à la survie
- Avoir des ambitions: motive les actions
- Se reproduire: nécessite deux estajoj de sexes différents

## 3. Système d'historisation
- Stockage JSON
- Format: simulation_YYYYMMDD_HHMMSS.json
- Contenu:
  - ID de simulation
  - Timestamp
  - Paramètres
  - Liste d'événements

## 4. Interface utilisateur (TUI)
- 4 panneaux d'affichage:
  - Population (total, males, females)
  - Log des événements
  - État des besoins
  - Détails de l'Estajo sélectionné
- Commandes:
  - q/Esc: quitter
  - p: pause
  - ←/→: navigation entre Estajoj

## 5. Environnement technique
- Langage: Rust
- IDE: Visual Studio Code
- OS: Ubuntu
- Tests unitaires