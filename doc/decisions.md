# Décisions :

- [x] On choisit l'exploration spatiale car on aime bien le style dépouillé, situation désespérée. On pense que ce thème
  simple est adapté à notre gameplay et reste à notre portée en terme de faisabilité.
- [x] Le personnage peut utiliser un mélange de technologie et de magie. Magie/Tech
- [x] les ennemis en mourrant, tombent du loot
- [x] la carte est un planète sphérique qui se répète sur laquelle on tourne à l'infinie. Pas d'obstacles sur la planète
  pour le moment.
- [x] pas de brouillard de guerre : car pas justifié par le gameplay (surtout sur une carte plate). C'est aussi plus
  simple à gérer pour nous.
- [x] on arrive sur la planète, on survie sur cette planète hostile jusqu'à ce qu'on vienne nous chercher. Tant que je
  ne suis pas exfiltré, si je meurs je perds les gains de la partie en cours (je ne perds pas ma metaprogression).
- [x] On choisit à l'avance le temps d'exfiltration (en minutes) : fait varier les récompenses en cas de réussite ? On
  est exfiltré dès que le temps est écoulé.
- [x] Meta progression (à définir)
- [x] Event aléatoires : boss qui pop
- [x] minimap sphérique ou à plat ou un radar avec un point de là où on se trouve et le vaisseau crashé (comme
  référence)

Intéractions :

- [x] WASD/Joystick gauche : déplacements du perso
- [x] Souris/Joystick droit : direction de la visée
- [x] indicateur de la direction du personnage : orientation du sprite
- [x] indicateur de la direction de la visée : une prévisualisation de la zone d'effet de l'arme actuelle
- [x] De préférence les armes soint en tir auto (pour éviter l'effet pianotage)
- [x] Possibilité d'avoir des actifs comme une grenade (par exemple) que l'on déclenche manuellement et qui se balance
  dans la direction de la visée.
- [x] pas de notion d'actif consomamble (pour éviter la gestion de stock et le travail associé)

# Histoire/Narration

Comment on explique la boucle de : j'arrive sur une planète, je me fait exfiltrer puis je recommence ? Au niveau de la
narration

Propositions:

- On peut imaginer jouer avec plusieurs explications du pourquoi:
    - C'est le seul pilote disponible mais en fait il a eu brevet en trichant : il est nul
    - Son équipe le déteste et l'envoie toujours en mission en sabotant son vaisseau
    - Un artefact a capturé son empreinte mentale et le force à revenir sans cesse sur la planète et revrivre le même
      crash
    - ...
- Et distiller dans la narration des indices sur la vraie raison mais aussi des fausses pistes

Quelle est la vraie raison ?

# Structure du jeu

- 1 planète
- 1 personnage
- 1 arme de départ
- 2 slots d'actifs (au départ)
- le premier slot est rempli avec l'arme de départ
- le deuxième slot est vide
- pas de slots de passifs.

- [x] Quand on passe un LEVEL : on propose un actif ou une amélioration d'actif (que j'ai déjà) mais en complet RANDOM (
  côté casino qui donne envie de toujours voir après). On propose 3 choix random. Je peux dire que ça m'interesse pas et
  je pers la moitié de mon XP sans rien prendre. Permet de garder un slot vide.
- [x] Slot définitif (pour ne pas épuiser les armes ça parait plus réalisable)

# Boucle de jeu (pas meta):

1. Je choisis mon temps d'exfiltration (5,10,20,30...)
2. J'arrive sur une planète
3. Je commence à XP = 0
4. Des hordes d'enemis arrivent dans un rythme (crescendo)
5. Je tue des ennemis qui me fonce dessus constamment
6. On tue un ennemi on collecte de l'XP
7. Objectif principal : Je dois survivre jusqu'au temps impartie
8. Arriver à un seuil d'XP : level-up : choix amélioration : uniquement des actifs
9. Mon XP repasse à 0 : le prochain seuil est augmentée de manière exponentielle et proportionnelle à l'augmentation de
   la difficulté (A régler)
10. Si je me fait exfilter : je gagne de l'argent.
11. Si je meurs : je perds tout mon argent.

# Exploration/Events aléatoires :

- [x] 2 minute avant l'exfiltration, je vois sur la minimap l'endroit où elle va arriver. Je dois me rendre à cet
  endroit pour être exfilté au bon moment sinon je loupe l'exfiltration : fin immédiate.
- [x] Au fur et à mesure la zone d'exfiltration se précise en un point.
- [] On veut bien un event aléatoire de crash de vaisseau et qui me permette de voler son arme me permettant de changer
  d'arme dans un slot.
- [] Un boss pop

Sur quels paramètres on gère le rythme des ennemis :

- nb d'ennemis par secondes
- dans quelles directions
- quel types d'ennemis
  Comment augmente ses indicateurs ? Sur le temps ou l'XP ou les deux ?

# Questions ??

- Si j'ai mes 2 slots utilisés, je gagne de l'XP que se passe-t-il ? Vu que mes slots sont remplis et définitifs ?
  Qu'est-ce qu'on me propose comme amélioration ?
- Fusion d'arme pour libérer un slot ?
- [ ] Est-ce qu'on garde les events aléatoires : un autre vaisseau se crash (loot). Si oui pourquoi ? Ce serai une
  récompense immediate qui apparait à un point de la carte. Cela pousserai à l'exploration
- [ ] Est-ce que l'exfiltration marche dès que le temps est écoulé ?

# Affichage :

- compteur de temps
- barre d'XP
- barre de vie (au dessus du perso)
- 2 slots d'armes
- Comment je vois quels passifs j'ai
- Player 32x32

# Définitions des items

Voir les fichiers dédiés :

- [Personnages](player.md)
- [Ennemis](enemy.md)
- [Armes](weapon.md)
- [Passifs](passive.md)

# Sauvegarde du jeu

Comment ça se passe ?

# Menu meta progression

- sauver un compteur avec l'argent du joueur et les passifs déjà achetés
- comment est décrit le magasin
