# Roadmap

Ordre des prochaines grosses étapes (défini le 2026-04-14).

## 1. XP / Level up : [#5](https://github.com/agjini/colonie-terminus/issues/5)

- Afficher des gems XP de différentes valeurs suivant certains critères (type d'ennemi, chance)
- Pickup XP, increase XP, level up
- Autre loot ?

## 2. Réflexion sur l'extraction : [#6](https://github.com/agjini/colonie-terminus/issues/6)

- Décider de comment ça pourrait se passer
- Réfléchir si ça peut être intéressant niveau gameplay
- Piste : extraction demandée par le joueur, déclenche un compte à rebours et un déplacement souhaité

## 3. Impl actifs (armes) : [#12](https://github.com/agjini/colonie-terminus/issues/12)

- Afficher les slots d'armes avec les infos dedans (sprites, level)

## 4. Impl passifs : [#13](https://github.com/agjini/colonie-terminus/issues/13)

- Définir différents critères sur lesquels les passifs peuvent jouer (angle aim zone, vitesse de déplacement, santé max,
  chance de coup critique...)
- Lister chaque critère et implémenter les effets sur chacun
- Créer une liste de passifs dans un fichier RON
- Trouver des sprites pour les représenter

## 5. Choix au level up : [#7](https://github.com/agjini/colonie-terminus/issues/7)

- À chaque level up, proposer 3 items parmi les actifs et passifs
- Fournir des exemples d'items
- Implémenter les items

## 6. Ennemis et progression des vagues : [#9](https://github.com/agjini/colonie-terminus/issues/9), [#8](https://github.com/agjini/colonie-terminus/issues/8)

- Créer et décliner des ennemis
- Implémenter la progressivité dans les vagues selon le temps et/ou autre chose
- Tester différentes fonctions (exponentielle, log, autre)
- Sprites animations

## 7. Events spéciaux

- Boss
- ...

## 8. Méta-progression : [#10](https://github.com/agjini/colonie-terminus/issues/10)

- Gérer la "monnaie" et la méta-progression
- Créer le magasin
- Créer les items à vendre dedans
- Gérer le gain ou la perte (multiplicateur) d'argent lorsqu'on réussit une extraction ou qu'on meurt

## 9. Minimap : [#14](https://github.com/agjini/colonie-terminus/issues/14)

- Afficher le lieu du crash de la capsule
- Afficher la mini-map avec le lieu du crash
- Afficher la zone d'extraction sur la carte
