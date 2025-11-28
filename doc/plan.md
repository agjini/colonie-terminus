# Plan de développement MVP - Colonie Terminus

## Vision
Créer un survivor-like avec système d'énergie et visée orbitale unique. MVP jouable en 10 minutes avec fun immédiat.

## Contraintes critiques
- ❌ ZERO custom asset (formes géométriques uniquement)
- ❌ Max 2 semaines par feature
- ✅ Code clean, minimal, bien structuré
- ✅ Config dans config.ron
- ✅ Tester régulièrement

---

## Phase 1 : Fondations (Semaine 1-2)

### 1.1 Configuration et architecture
**Objectif** : Structure propre pour tout le reste

- [ ] Créer `assets/config/config.ron` avec toutes les valeurs de balance
  - Stats player (HP: 100, speed: 140, max_energy: 100, regen: 20/s)
  - Stats armes (Blaster, Plasma Launcher)
  - Stats ennemis (Grouilleur, Voltigeur, Prédateur)
  - Stats boss et events
  - Paramètres monde sphérique
- [ ] Créer système de chargement config (RON deserialize)
- [ ] Créer module `src/game/mod.rs` pour logique gameplay
- [ ] Nettoyer le code demo (garder structure, supprimer ducky)
- [ ] Setup States : Menu, InGame, GameOver, Exfiltration

**Test** : Config se charge sans erreur, states fonctionnent

### 1.2 Système d'énergie (CRITIQUE)
**Objectif** : Le cœur différenciateur du jeu

- [ ] Créer composant `Energy { current: f32, max: f32, regen_rate: f32 }`
- [ ] Créer ressource `EnergyConfig` (depuis config.ron)
- [ ] Système de régénération d'énergie (20/s par défaut)
- [ ] UI barre d'énergie (rectangle cyan qui pulse)
- [ ] Système de consommation d'énergie (pour futur système d'armes)

**Test** : Barre d'énergie se vide et se remplit, visuel pulse fonctionne

---

## Phase 2 : Combat de base (Semaine 3-4)

### 2.1 Système de visée orbitale (UNIQUE)
**Objectif** : Mécanique signature du jeu

- [ ] Créer composant `OrbitalCursor { angle: f32, radius: f32 }`
- [ ] Système input souris → angle sur cercle (150px radius)
- [ ] Support gamepad (joystick droit → angle)
- [ ] Afficher cursor (croix blanche 16x16) sur le cercle
- [ ] Option : cercle guide (outline blanc) montrant la trajectoire

**Test** : Cursor suit souris/gamepad fluidement, reste sur cercle

### 2.2 Arme de départ : Blaster
**Objectif** : Première arme fonctionnelle

- [ ] Créer composant `Weapon { damage, energy_cost, cooldown, ... }`
- [ ] Composant `Projectile { damage, speed, lifetime, ... }`
- [ ] Système de tir automatique vers cursor
  - Check énergie disponible
  - Consomme énergie
  - Spawn projectile (rectangle bleu 4x2)
- [ ] Système mouvement projectiles
- [ ] Système collision projectiles (despawn à l'impact)
- [ ] Cooldown entre tirs (0.2s)

**Test** : Blaster tire auto vers cursor, coûte énergie, projectiles volent

### 2.3 Premier ennemi : Grouilleur
**Objectif** : Valider combat de base

- [ ] Créer composant `Enemy { hp, damage, speed, enemy_type }`
- [ ] Créer `EnemyType` enum (Crawler, Flyer, Predator)
- [ ] Spawn Grouilleur (carré rouge 24x24) hors écran
- [ ] IA simple : move vers player
- [ ] Système collision projectile → ennemi (damage, despawn)
- [ ] Système collision ennemi → player (damage player)
- [ ] Ennemi drop XP à la mort (orbe violet)

**Test** : Grouilleur apparaît, poursuit, meurt au tir, drop XP

---

## Phase 3 : Boucle XP et progression (Semaine 5)

### 3.1 Système XP et level-up
**Objectif** : Boucle de progression

- [ ] Composant `Experience { current: f32, required: f32, level: u32 }`
- [ ] Système collect XP (collision player → XP orb)
- [ ] Système level-up (toutes les 30-60s selon balance)
- [ ] UI barre XP (rectangle violet)
- [ ] Event `LevelUpEvent` pour triggerer choix upgrade

**Test** : Tuer ennemis → XP → level up après X kills

### 3.2 Système d'upgrades (choix de cartes)
**Objectif** : Choix tactiques

- [ ] Créer écran pause lors level-up
- [ ] Afficher 3 cartes d'upgrade aléatoires
  - Nouvelle arme (si <2 slots)
  - +Dégâts arme actuelle
  - +Régénération énergie
  - +Vitesse mouvement
  - +HP max
  - +Rayon collecte XP
- [ ] Appliquer upgrade choisi
- [ ] Retour au jeu

**Test** : Level up → 3 choix → choix appliqué → jeu continue

---

## Phase 4 : Carte sphérique (Semaine 6)

### 4.1 Monde sphérique
**Objectif** : Carte infinie sans bords

- [ ] Ressource `SphericalWorld { radius: f32, circumference: f32 }`
- [ ] Système wrapping (entités sortent à gauche → réapparaissent à droite)
- [ ] Wrapping vertical aussi
- [ ] Adapter spawn ennemis (apparaissent de toutes directions)

**Test** : Player/ennemis traversent bords sans rupture visuelle

### 4.2 Minimap sphérique
**Objectif** : Navigation sur planète

- [ ] UI cercle (minimap) en haut à droite
- [ ] Point blanc = position player
- [ ] Point rouge = position vaisseau crashé (point de référence)
- [ ] Mise à jour en temps réel

**Test** : Minimap reflète position précise sur planète

---

## Phase 5 : Contenu MVP (Semaine 7-8)

### 5.1 Armes complètes
**Objectif** : 2 armes jouables

- [ ] Plasma Launcher (2ème arme)
  - Coût : 30 énergie
  - Dégâts : 50 AoE
  - Projectile rouge + explosion (cercle rouge expanding)
- [ ] Système multi-armes (switch ou tir simultané ?)
- [ ] UI slots d'armes (2 rectangles montrant armes équipées)

**Test** : 2 armes différentes fonctionnelles, balance intéressante

### 5.2 Ennemis complets
**Objectif** : 3 types d'ennemis variés

- [ ] Voltigeur (triangle vert)
  - Vitesse moyenne
  - Mouvement erratique (zigzag)
- [ ] Prédateur (losange orange)
  - Très rapide
  - Plus de HP et dégâts
- [ ] Système spawn progressif
  - Niveaux 1-3 : Surtout Grouilleurs
  - Niveaux 3-5 : Mélange
  - Niveaux 6+ : Tous types, scaling +15% HP/30s

**Test** : Difficulté croît naturellement, variété intéressante

---

## Phase 6 : Events et timer (Semaine 9)

### 6.1 Boss event
**Objectif** : Moment épique aléatoire

- [ ] Boss "Gardien Planétaire" (hexagone rouge 64x64)
  - 500 HP, 2 phases
  - Phase 1 : Charge lente + projectiles simples
  - Phase 2 (<50% HP) : Plus rapide, 3 projectiles en éventail
- [ ] Event aléatoire (20% chance tous les 3 niveaux)
- [ ] Drop loot rare à la mort
- [ ] Annonce visuelle/sonore du spawn

**Test** : Boss apparaît, combat challengeant, loot récompense

### 6.2 Crashed ship event
**Objectif** : Event alternatif

- [ ] Spawn vaisseau crashé (triangle gris) à position aléatoire
- [ ] 18 ennemis autour du vaisseau
- [ ] 3 orbes de loot au centre
- [ ] Marker sur minimap
- [ ] 15% chance tous les 2 niveaux

**Test** : Event apparaît, combat intéressant, loot visible

### 6.3 Timer d'exfiltration
**Objectif** : Tension et choix risque/récompense

- [ ] Menu départ : choix durée (5, 10, 20, 30 min)
- [ ] Timer compte à rebours
- [ ] UI timer visible (coin haut gauche)
- [ ] À 0:00 → écran exfiltration success
- [ ] Mort avant → écran game over

**Test** : Timer fonctionne, fin de partie selon condition

---

## Phase 7 : UI et feedback (Semaine 10)

### 7.1 UI complète
**Objectif** : Clarté des infos

- [ ] Barre HP (rectangle rouge au-dessus player)
- [ ] Barre énergie (rectangle cyan, pulse glow)
- [ ] Barre XP (rectangle violet, bas écran)
- [ ] Timer (texte blanc, haut gauche)
- [ ] Minimap (cercle, haut droite)
- [ ] Slots armes (2 rectangles, bas gauche)
- [ ] Niveau actuel (texte blanc, haut centre)

**Test** : Toutes infos lisibles pendant gameplay intense

### 7.2 Feedback visuel
**Objectif** : Jeu feels good

- [ ] Particules mort ennemi (explosion blanche)
- [ ] Flash rouge damage player
- [ ] Shake caméra sur gros impacts
- [ ] Glow projectiles
- [ ] Pulse barre énergie selon niveau

**Test** : Impacts satisfaisants, infos claires

---

## Phase 8 : Balance et polish (Semaine 11-12)

### 8.1 Balance gameplay
**Objectif** : Fun immédiat

- [ ] Ajuster valeurs dans config.ron
  - Régénération énergie (fun vs frustrant ?)
  - Coûts armes (choix tactiques ?)
  - HP ennemis (challenge correct ?)
  - Fréquence level-up (tous les 30-60s ?)
- [ ] Scaling difficulté (courbe exponentielle correcte ?)
- [ ] Spawn events (fréquence fun ?)

**Test interne** : 10 min de jeu = fun ? Envie de rejouer ?

### 8.2 Audio basique
**Objectif** : Ambiance minimale

- [ ] 1 track ambient (loop) - asset gratuit
- [ ] 1 track boss/event - asset gratuit
- [ ] SFX tir (blip simple)
- [ ] SFX impact (pop simple)
- [ ] SFX level-up (ding)
- [ ] SFX death (explosion)

**Test** : Audio améliore expérience sans être gênant

### 8.3 Écrans de jeu
**Objectif** : Boucle complète

- [ ] Menu principal
  - Bouton Play
  - Choix durée exfiltration
  - Bouton Quit
- [ ] Écran Game Over
  - Stats run (kills, niveau atteint, temps survécu)
  - Bouton Retry
  - Bouton Menu
- [ ] Écran Exfiltration Success
  - Stats run
  - Ressources gagnées (pour future meta-progression)
  - Bouton Retry
  - Bouton Menu

**Test** : Boucle Menu → Jeu → Fin → Menu fonctionne

---

## Phase 9 : Testing externe (Semaine 13)

### 9.1 Playtest avec 3 personnes externes
**Questions critiques** :
1. Après 10 min, envie de rejouer ? (≥2/3 doivent dire oui)
2. Énergie intéressante ou frustrante ?
3. Events excitants ?
4. Carte sphérique intuitive ?
5. Sensation de progression ?

### 9.2 Ajustements post-playtest
- [ ] Identifier 3 problèmes principaux
- [ ] Fix rapides (1 semaine max)
- [ ] Re-test si changements majeurs

---

## Ordre des systèmes Bevy (architecture finale)

```rust
Update Schedule:
1. input_system
2. orbital_cursor_system (update cursor sur cercle)
3. energy_regen_system
4. weapon_system (tir auto, energy check)
5. movement_system
6. spherical_world_wrapping_system
7. projectile_system
8. collision_system (projectile/enemy, enemy/player)
9. enemy_ai_system
10. spawn_system (ennemis)
11. event_system (boss, crashed ship)
12. xp_collection_system
13. level_up_system
14. ui_system (draw tout)
```

---

## Structure fichiers cible

```
src/
├── main.rs
├── game/
│   ├── mod.rs
│   ├── config.rs (load config.ron)
│   ├── player/
│   │   ├── mod.rs
│   │   ├── movement.rs
│   │   ├── energy.rs (système énergie)
│   │   └── stats.rs
│   ├── combat/
│   │   ├── mod.rs
│   │   ├── orbital_cursor.rs (visée)
│   │   ├── weapons.rs
│   │   └── projectiles.rs
│   ├── enemies/
│   │   ├── mod.rs
│   │   ├── types.rs (Crawler, Flyer, Predator)
│   │   ├── ai.rs
│   │   └── spawning.rs
│   ├── progression/
│   │   ├── mod.rs
│   │   ├── experience.rs
│   │   ├── level_up.rs
│   │   └── upgrades.rs
│   ├── events/
│   │   ├── mod.rs
│   │   ├── boss.rs
│   │   └── crashed_ship.rs
│   ├── world/
│   │   ├── mod.rs
│   │   └── spherical.rs
│   └── ui/
│       ├── mod.rs
│       ├── hud.rs (HP, energy, XP bars)
│       ├── minimap.rs
│       └── level_up_screen.rs
├── screens/ (existant, adapter)
└── menus/ (existant, adapter)

assets/
├── config/
│   └── config.ron (TOUT le balance ici)
├── audio/
│   ├── music/ (assets gratuits)
│   └── sfx/ (assets gratuits)
└── images/ (placeholder si besoin)
```

---

## Métriques de succès MVP

### Critères techniques
- [ ] 60 FPS stable avec 100+ ennemis à l'écran
- [ ] <50ms input lag (cursor, tir)
- [ ] 0 crash sur 30 min de jeu
- [ ] Wrapping sphérique sans glitch visuel

### Critères gameplay
- [ ] Système énergie = choix tactiques (pas juste spam)
- [ ] Events random = moments excitants
- [ ] Progression = sensation de puissance croissante
- [ ] 10 min de jeu = boucle addictive

### Critères validation
- [ ] 3/3 testeurs externes veulent rejouer
- [ ] Énergie jugée "intéressante" par 2/3 testeurs
- [ ] Events jugés "excitants" par 2/3 testeurs
- [ ] Carte sphérique "intuitive" pour 2/3 testeurs

---

## Red flags à surveiller

🚩 **Scope creep** : Envie d'ajouter features hors MVP
→ Réponse : Noter dans "Level 2 ideas", finir MVP d'abord

🚩 **Perfectionnisme assets** : Passer >2j sur un sprite
→ Réponse : Formes géométriques suffisent pour MVP

🚩 **Over-engineering** : Système trop complexe "pour le futur"
→ Réponse : KISS, refactor plus tard si besoin

🚩 **Pas de playtest pendant 2+ semaines**
→ Réponse : Jouer 10 min CHAQUE semaine minimum

🚩 **Feature bloquée >1 semaine**
→ Réponse : Simplifier ou découper en sous-tâches

---

## Notes importantes

### Philosophie de développement
- **Clean code** : Refactor au fur et à mesure, pas de dette technique
- **Minimal** : Supprimer commentaires inutiles, code auto-documenté
- **Config first** : Tout le balance dans config.ron pour iteration rapide
- **Test early** : Tester après chaque phase, pas attendre la fin

### Mantra
1. "Est-ce que ça rend le jeu plus FUN ?" → Si non, cut
2. "MVP first" → Features Level 2+ attendent
3. "Tester avant de coder plus"
4. "Config file > hardcode"
5. "Clean code = fast code in the long run"

### Post-MVP (NE PAS FAIRE AVANT FIN MVP)
- Meta-progression avec shop
- Synergies tech/magie
- 2ème personnage
- Plus d'armes (10 total)
- Plus de passifs (25 total)
- Animations complexes
- Shaders custom
- Multiple biomes

---

## Timeline réaliste

| Semaine | Phase | Milestone |
|---------|-------|-----------|
| 1-2 | Fondations | Config + énergie fonctionnels |
| 3-4 | Combat base | Blaster + Grouilleur + visée orbitale |
| 5 | Progression | XP + level-up + upgrades |
| 6 | Monde | Carte sphérique + minimap |
| 7-8 | Contenu | 2 armes + 3 ennemis |
| 9 | Events | Boss + crashed ship + timer |
| 10 | UI/Feedback | Interface complète + particules |
| 11-12 | Balance | Ajustements + audio + écrans |
| 13 | Testing | Playtest externe + fixes |

**Total : ~3 mois** (compatible avec contraintes)

---

## Checkpoint decision points

### Fin Phase 2 (Semaine 4)
**Question** : Visée orbitale + énergie = fun ?
- Si OUI → Continuer
- Si NON → Revoir mécaniques core AVANT d'aller plus loin

### Fin Phase 5 (Semaine 8)
**Question** : Boucle combat/XP/upgrade addictive ?
- Si OUI → Continuer vers events
- Si NON → Ajuster balance avant events

### Fin Phase 9 (Semaine 13)
**Question** : 3 testeurs externes validés ?
- Si OUI → MVP terminé, planifier Level 2
- Si NON → Identifier problème, fix, re-test (max 2 semaines)

---

## En cas de blocage

**Si une feature prend >2 semaines** :
1. Découper en plus petites tâches
2. Simplifier l'approche
3. Remplacer par version géométrique/placeholder
4. Si toujours bloqué → demander aide ou skip temporairement

**Si perte de motivation** :
1. Jouer au jeu (même incomplet)
2. Regarder progression depuis début
3. Lire les mantras
4. Se rappeler : finir MVP > perfect game

---

## Prochaine étape immédiate

**ACTION 1** : Créer `assets/config/config.ron` avec toutes les valeurs de balance
**ACTION 2** : Créer module `src/game/config.rs` pour charger le fichier
**ACTION 3** : Implémenter système énergie (player + UI + regen)

Une fois l'énergie fonctionnelle et que ça feels good → Phase 2 (combat).
