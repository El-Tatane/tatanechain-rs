# Wallet (gestion des clés)
- [X] GenerateWallet — crée une nouvelle paire de clés
- [ ] ImportWalletFromPrivateKey — reconstruit un wallet depuis une clé privée brute (32 octets hex)
- [ ] ImportWalletFromMnemonic — reconstruit un wallet depuis une phrase mnémonique BIP-39 (12/24 mots → seed → clé)

- [ ] SignTransaction — signe un payload avec la clé privée du wallet
- [ ] GetAddress — dérive l'adresse depuis la clé publique

# Transaction (côté utilisateur/client)
- [ ] CreateTransaction — construit une tx non signée à partir de (sender, recipient, amount, nonce)
- [ ] SubmitTransaction — point d'entrée public : valide crypto + règles métier, ajoute au mempool, broadcast
- [ ] GetTransactionStatus — cherche une tx par hash (pending dans le mempool, confirmée dans un block, inconnue)

# Mempool
- [ ] AddToMempool — après validation
- [ ] SelectTransactionsForBlock — pioche les N tx à inclure (tri par fee décroissante, ordre des nonces par sender respecté)
- [ ] PurgeMinedTransactions — nettoie après qu'un block a été accepté
- [ ] EvictStaleTransactions — TTL ou nonce périmé

# Mining / Production de blocks
- [ ] BuildCandidateBlock — assemble header (previous_hash, merkle_root, timestamp) + transactions choisies
- [ ] MineBlock — boucle PoW : incrémente le nonce jusqu'à trouver un hash valide
- [ ] ProposeBlock — broadcast le block miné aux pairs

# Validation & chaîne
- [ ] ValidateBlock — hash correct, PoW respecte la difficulté, previous_hash chaîne bien, toutes les tx sont valides, pas de double-spend intra-block
- [ ] AppendBlock — ajoute à la chaîne, applique les changements d'état, purge le mempool
- [ ] HandleIncomingBlock — le use case appelé quand un pair envoie un block (valide → append, ou déclenche une réorg)
- [ ] ReorganizeChain — quand une chaîne concurrente devient plus longue : rollback des blocks locaux, replay des blocks de la nouvelle branche
- [ ] ValidateFullChain — replay from genesis, utile au bootstrap ou pour un audit

# Requêtes de lecture (state)
- [ ] GetBalance(address) — lit l'état courant
- [ ] GetBlockByHash / GetBlockByHeight
- [ ] GetChainHead
- [ ] GetTransactionHistory(address) — nécessite un index secondaire

# Réseau (P2P)
- [ ] DiscoverPeers — bootstrap depuis une liste seed, puis échange de peer lists
- [ ] ConnectToPeer — handshake, échange de version/height
- [ ] SyncChain — quand ton height est inférieur à celui d'un pair : demande les blocks manquants par batch
- [ ] BroadcastTransaction / BroadcastBlock — gossip aux pairs connectés
- [ ] HandleIncomingTransaction — reçoit d'un pair, valide, ajoute au mempool, rebroadcast si nouveau

# Node lifecycle
- [ ] StartNode — charge la chaîne persistée, initialise mempool/state/network, démarre les tâches (miner, P2P, RPC)
- [ ] PersistBlock — écrit sur disque à chaque append (RocksDB, sled, ou simple fichier pour apprendre)
- [ ] LoadChainFromDisk — au démarrage
- [ ] Ordre d'implémentation suggéré

Pour un projet d'apprentissage, je démarrerais mono-nœud sans réseau d'abord :

Wallet (Generate, Sign) + Transaction (Create, Validate)
Block + Blockchain en mémoire + AppendBlock avec validation basique
Mempool + BuildCandidateBlock + MineBlock (PoW simple, difficulté basse)
Gestion du state (balances) + GetBalance + rejet des tx sans fonds
Persistance disque
Puis couche P2P (le plus gros morceau, à isoler) : sync, gossip, réorg
Finalement RPC/HTTP pour exposer les use cases à un client