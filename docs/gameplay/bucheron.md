# Le bûcheron

Il abat des arbres et produit du **bois**. Ce métier est **en place** : forêts, repousse, abattage à plusieurs et livraison à la main au coffre de la cabane.

## Le principe — décidé

- Le bûcheron coupe des **arbres** pour en tirer du bois.
- Le bois sert à **construire** : c'est la matière des chantiers de l'[artisan](artisan.md).
- Le bois est ensuite acheminé par les [transporteurs](transporteur.md).

## Les arbres et les forêts — décidé

- Les **arbres poussent** : couper un arbre ne l'élimine pas pour toujours.
- Il y a des **zones de forêt**, et elles sont **limitées**. C'est la différence avec les filons, qui sont infinis : le bois est une ressource qui peut manquer si on coupe plus vite que ça ne repousse.
- Le bûcheron travaille donc dans les forêts, et le royaume doit les ménager.

## En place aujourd'hui

- **Deux forêts** dans le royaume (nord-ouest et sud-ouest), chacune avec sa cabane et son bûcheron. Une forêt est une zone d'arbres avec un couloir libre toutes les trois colonnes. Les arbres isolés hors forêt restent du décor : on ne les coupe pas.
- Un arbre se coupe depuis une case voisine en le regardant (6 s pour l'IA, 3 s pour le joueur) et donne **3 bûches**. À sa place reste une **pousse**, qui redevient un arbre après **une journée de jeu** (24 h). Une forêt ne s'étend jamais : on peut la vider en coupant plus vite que ça ne repousse.
- L'IA coupe l'arbre le plus proche de sa forêt, même si un autre bûcheron y est déjà. **À plusieurs, les coups s'additionnent** : l'arbre tombe plus vite, et la progression est conservée si l'un s'arrête. Seul celui qui porte **le dernier coup** reçoit le bois ; les autres repartent vers un autre arbre.
- Il porte son bois **à la main** (9 bûches au plus, soit trois arbres) jusqu'au **coffre de sa cabane**, posé à côté du bâtiment. S'il est couché avec du bois, il le livre à son réveil. On voit sur lui un bloc de bois et le nombre de bûches. Il ne coupe plus si ses mains ne peuvent pas recevoir un arbre de plus.
- Si le coffre de la cabane est plein (200), il y dépose ce qui rentre et **attend à côté** avec le reste, sans couper, jusqu'à ce qu'un [transporteur](transporteur.md) en emporte. Le joueur dépose avec la touche d'action.

## À définir

- La **vitesse de repousse** d'un arbre, comparée à la vitesse de coupe : c'est elle qui décide si la forêt tient ou s'épuise.
- La **taille** et le nombre des zones de forêt. Les zones de la carte ne sont pas encore dimensionnées (voir [royaume.md](royaume.md)).
- Le roi peut-il **désigner** une forêt à exploiter ou à ménager ?
- Faut-il des **outils** pour couper (voir [forgeron.md](forgeron.md)) ?
- Y aura-t-il une **replantation** à la main ? Pour l'instant l'arbre repousse seul.
