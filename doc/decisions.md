# Questions/Décisions :
- [x] On choisit l'exploration spatiale car on aime bien le style dépouillé, situation désespérée. On pense que ce thème simple est adapté à notre gameplay et reste à notre portée en terme de faisabilité.
- [x] Le personnage peut utiliser un mélange de technologie et de magie. Magie/Tech
- [x] les ennemis en mourrant, tombent du loot
- [x] la carte est un planète sphérique qui se répète sur laquelle on tourne à l'infinie. Pas d'obstacles sur la planète pour le moment.
- [x] pas de brouillard de guerre : car pas justifié par le gameplay (surtout sur une carte plate). C'est aussi plus simple à gérer pour nous.
- [x] on arrive sur la planète, on survie sur cette planète hostile jusqu'à ce qu'on vienne nous chercher. Tant que je ne suis pas exfiltré, si je meurs je perds les gains de la partie en cours (je ne perds pas ma metaprogression).
- [x] On choisit à l'avance le temps d'exfiltration (en minutes) : fait varier les récompenses en cas de réussite ? On est exfiltré dès que le temps est écoulé.



- [ ] Meta progression à définir. Quel est kle collectable comment il agit sur nous : XP perso, equipements, passifs ?
- [x] Event aléatoires : boss qui pop
- [ ] Event aléatoires : un autre vaisseau se crash (loot). Si oui pourquoi ? Ce serai une récompense immediate qui apparait à un point de la carte
- [ ] minimap sphérique ou à plat ou un radar avec un point de là où on se trouve et le vaisseau crashé (comme référence)

Intéractions :
- [x] WASD/Joystick gauche : déplacements du perso
- [x] Souris/Joystick droit : direction de la visée
- [x] indicateur de la direction du personnage : orientation du sprite
- [x] indicateur de la direction de la visée : une prévisualisation de la zone d'effet de l'arme actuelle
- [x] De préférence les armes soint en tir auto (pour éviter l'effet pianotage)
- [x] Possibilité d'avoir des actifs comme une grenade (par exemple) que l'on déclenche manuellement et qui se balance dans la direction de la visée

- comment on explique la boucle de : j'arrive sur une planète, je me fait exfiltrer puis je recommence ?



- 1 planète
- 1 personnage
- 1 arme de départ

- 2 slots d'actifs (au départ)
- le premier slot est rempli avec l'arme de départ
- le deuxième slot est vide

- [x] Quand on passe un LEVEL : on propose un actif ou une amélioration d'actif (que j'ai déjà) mais en complet RANDOM (cote casino). On propose 3 choix random.  Je peux dire que ça m'interesse pas et je pers la moitié de mon XP
- [x] Slot définitif (pour ne pas épuiser les armes ça parait plus réalisable)


Boucle de jeu (pas meta):
1. Je choisis mon temps d'exfiltration (5,10,20,30...)
2. J'arrive sur une planète
1. Je commence à XP = 0
3. Des hordes d'enemis arrivent dans un rythme (crescendo)
3. Je tue des ennemis qui me fonce dessus constamment
4. On tue un ennemi on collecte de l'XP
3. Objectif principal : Je dois survivre jusqu'au temps impartie
- Arriver à un seuil d'XP : level-up : choix amélioration : uniquement des actifs
- Mon XP repasse à 0 : le prochain seuil est augmentée de manière exponentielle et proportionnelle à l'augmentation de la difficulté (A régler)
- Si je me fait exfilter : je gagne de l'argent.

Rythme d'arrivée des ennemis à définir :
- ennemis par secondes
- dans quelles directions
-
