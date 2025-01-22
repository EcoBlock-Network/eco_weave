# eco_weave


Téléphone A (création transaction)
    ↓
Transaction créée et signée
    ↓
Validation locale
    ↓
Ajout au Tangle local (statut : pending)
    ↓
Propagation asynchrone aux voisins
    ↓
Validation chez les voisins
    ↓
Ajout au Tangle des voisins (statut : pending)
    ↓
Consensus local
    ↓
Propagation de la confirmation
    ↓
Transaction confirmée