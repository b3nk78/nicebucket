# Tutoriel Sweetbucket 🍯✨

Bienvenue dans le guide complet d'utilisation de **Sweetbucket**, votre explorateur de stockage objet S3 rapide, sécurisé et convivial !

---

## Sommaire
1. [Introduction](#1-introduction)
2. [Premiers pas & Démarrage](#2-premiers-pas--démarrage)
3. [Configurer ses connexions](#3-configurer-ses-connexions)
   - [AWS S3 Standard](#a-connexion-aws-s3-standard)
   - [AWS S3 avec MFA (Multi-Factor Authentication)](#b-authentification-mfa)
   - [AWS IAM AssumeRole (Délégation de rôle STS)](#c-délégation-de-rôle-sts-assumerole)
   - [Cloudflare R2](#d-cloudflare-r2)
   - [S3 Compatible / MinIO / LocalStack / Scaleway](#e-fournisseurs-s3-compatibles-personnalisés)
4. [Gérer ses profils enregistrés](#4-gérer-ses-profils-enregistrés)
5. [Explorer et prévisualiser les buckets](#5-explorer-et-prévisualiser-les-buckets)
6. [Transférer des fichiers (Upload & Download)](#6-transférer-des-fichiers-upload--download)
7. [Organiser ses objets (Dossiers, Déplacement & Suppression)](#7-organiser-ses-objets)
8. [Le moniteur de transfert en direct](#8-le-moniteur-de-transfert-en-direct)
9. [Astuces & Dépannage fréquent](#9-astuces--dépannage-fréquent)

---

## 1. Introduction

**Sweetbucket** est une application de bureau multiplateforme (Windows, macOS, Linux) conçue avec **Tauri v2** et **React**. Elle permet de manipuler vos buckets de stockage objet sans passer par la console AWS ou des lignes de commande complexes, tout en intégrant des fonctionnalités d'authentification avancées et un moteur de transfert optimisé pour les gros volumes d'objets.

---

## 2. Premiers pas & Démarrage

### Lancement de l'application
- **Exécutable installé** : Lancez simplement l'application `sweetbucket` depuis votre menu Démarrer ou dossier Applications.
- **En environnement de développement** :
  ```bash
  npm run tauri dev
  ```

À l'ouverture, l'écran principal vous présente le formulaire de connexion à gauche (ou centré si aucune connexion n'est active) et la liste de vos connexions enregistrées.

---

## 3. Configurer ses connexions

Sweetbucket prend en charge plusieurs types de fournisseurs de stockage et modes d'authentification.

### A. Connexion AWS S3 standard
1. Dans le menu déroulant **Provider**, sélectionnez `S3`.
2. Donnez un **Label** à votre connexion (ex: *Mon Projet Production*).
3. Renseignez votre **Access Key ID** et votre **Secret Access Key**.
4. Cliquez sur **Connect** ou sur **Save Connection** pour la conserver dans le trousseau sécurisé de votre système.

### B. Authentification MFA
Si votre compte IAM exige une authentification multi-facteurs :
1. Renseignez l'ARN de votre appareil MFA dans le champ **MFA Device ARN** (ex: `arn:aws:iam::123456789012:mfa/mon-utilisateur`).
2. Lors du clic sur **Connect**, une boîte de dialogue s'ouvre pour saisir votre code à 6 chiffres.
3. 💡 *Astuce* : Vous pouvez valider directement votre code MFA avec la touche **Entrée** (`Enter`) sans avoir besoin de cliquer.

### C. Délégation de rôle STS (AssumeRole)
Particulièrement utile dans les architectures d'entreprise où un compte maître ("jumphost" ou rebond) assume un rôle vers un compte cible :
1. Configurez et enregistrez d'abord votre connexion maître (**Master Connection**) dans Sweetbucket.
2. Choisissez le provider **S3AssumeRole**.
3. Sélectionnez votre connexion maître dans la liste déroulante.
4. Indiquez le **Role ARN** cible à assumer (ex: `arn:aws:iam::987654321098:role/RoleAdminS3`).
5. *(Optionnel)* Renseignez la **Région AWS** (ex: `eu-west-3`, `us-east-1`).
6. Validez : Sweetbucket négociera les identifiants temporaires STS en toute transparence.

### D. Cloudflare R2
1. Sélectionnez le provider **R2**.
2. Indiquez votre **Account ID** Cloudflare (disponible dans votre tableau de bord R2).
3. Renseignez vos identifiants d'API R2 (Access Key ID et Secret Access Key).

### E. Fournisseurs S3 compatibles personnalisés
*(MinIO, Ceph, LocalStack, Wasabi, Scaleway, Exoscale...)*
1. Sélectionnez le provider **Custom**.
2. Indiquez l'URL complète de l'endpoint (ex: `http://localhost:4566` pour LocalStack ou `https://s3.fr-par.scw.cloud`).
3. Saisissez vos identifiants d'accès.

---

## 4. Gérer ses profils enregistrés

Toutes vos connexions enregistrées sont listées sous le formulaire :
- **Connexion rapide** : Cliquez sur **Connect** à côté d'un profil pour vous y connecter instantanément.
- **Édition d'un profil** : Cliquez sur le bouton **Edit**. Un bandeau jaune indique le mode édition et pré-remplit les champs (il suffit de ressaisir la clé secrète par sécurité). Cliquez sur **Update** pour sauvegarder ou **Cancel** pour annuler.
- **Suppression d'un profil** : Cliquez sur le bouton de corbeille rouge pour retirer un profil du trousseau système.

---

## 5. Explorer et prévisualiser les buckets

Une fois connecté, l'explorateur de fichiers s'affiche :

1. **Liste des buckets** : Cliquez sur le nom d'un bucket pour en afficher le contenu.
2. **Navigation** : 
   - Double-cliquez sur un dossier pour y entrer.
   - Utilisez le fil d'Ariane (*Breadcrumb*) en haut pour remonter rapidement dans l'arborescence.
   - Cliquez sur `..` pour revenir au dossier parent.
3. **Recherche instantanée** : Tapez quelques lettres dans le champ **Search...** en haut à droite pour filtrer les fichiers visibles en temps réel.
4. **Prévisualisation sans téléchargement** : Cliquez sur un fichier (image, texte, JSON, code...) pour ouvrir le volet de prévisualisation latérale. Vous y trouverez sa taille, sa date de modification et son aperçu direct.
5. **Copier l'URL d'un objet** : Cliquez sur les trois petits points `⋮` à droite d'un fichier puis sur **Copy Object Url**.

---

## 6. Transférer des fichiers (Upload & Download)

### Télécharger des fichiers (Download)
- **Fichier unique** : Menu `⋮` sur la ligne du fichier > **Download** > choisissez le dossier de destination.
- **Téléchargement multiple** : Cochez les cases à gauche de plusieurs fichiers, puis cliquez sur le bouton **Download** dans la barre d'actions flottante en bas.
- **Dossier complet en ZIP** : Menu `⋮` sur un dossier > **Download**. Sweetbucket rassemble l'ensemble des fichiers du dossier et génère automatiquement une archive `.zip`.

### Envoyer des fichiers (Upload)
1. Positionnez-vous dans le dossier de destination souhaité.
2. Assurez-vous qu'aucun fichier n'est coché pour afficher le bouton d'action principal.
3. Cliquez sur **Upload**.
4. Sélectionnez un ou plusieurs fichiers dans l'explorateur de votre système.
5. Le transfert démarre immédiatement.

---

## 7. Organiser ses objets

### Créer un dossier
1. Cliquez sur le bouton **+ Create Folder** en bas.
2. Saisissez le nom du dossier et validez.

### Déplacer des fichiers (Move)
1. Sélectionnez les fichiers à déplacer à l'aide des cases à cocher.
2. Cliquez sur le bouton **Move** dans la barre d'actions du bas.
3. Choisissez le dossier cible dans la boîte de dialogue et confirmez.

### Supprimer des fichiers ou dossiers (Delete)
- **Pour des fichiers** : Cochez les éléments puis cliquez sur **Delete** (ou via le menu `⋮` pour un fichier unique).
- **Pour un dossier** : Menu `⋮` > **Delete**.
- Une confirmation de sécurité vous demandera de valider l'action.

> 🚀 **Performance** : Grâce à l'optimisation S3 Batch intégrée dans Sweetbucket, la suppression de centaines ou milliers d'objets s'effectue par paquets de 1 000 clés simultanées, éliminant les lenteurs habituelles.

---

## 8. Le moniteur de transfert en direct

Lors de tout envoi ou téléchargement, un widget flottant apparaît automatiquement dans le coin inférieur droit :

- **Libellé de l'action** : *Uploading…* ou *Downloading…*
- **Compteur temps réel** : Affiche la progression des fichiers traités (ex: `14 / 85`).
- **Barre d'avancement & Pourcentage** : Visualisation fluide de l'état global.
- **Confirmation visuelle** : Dès que l'ensemble des fichiers est transféré, une coche verte s'affiche (*Upload complete* / *Download complete*) avant que le widget ne disparaisse automatiquement après 1,5 seconde.
- **Régulation automatique** : Pour éviter toute saturation de mémoire ou blocage réseau sur les gros volumes, Sweetbucket régule les flux avec un maximum de 8 transferts simultanés en parallèle.

---

## 9. Astuces & Dépannage fréquent

| Situation | Cause possible | Solution recommandée |
|---|---|---|
| **Erreur "No S3 bucket detected"** | Permissions IAM restreintes sur le compte ou compte maître dédié uniquement au rôle STS. | Normal si votre utilisateur IAM n'a pas la permission `s3:ListAllMyBuckets`. Utilisez une connexion AssumeRole vers le compte détenant les buckets. |
| **Erreur de région / endpoint** | Région vide ou différente de celle du bucket. | Spécifiez explicitement la région dans la configuration de votre connexion (ex: `eu-west-3`). |
| **Token MFA expiré** | Les sessions STS avec MFA ont une durée de vie limitée par AWS (souvent 1h à 12h). | Reconnectez-vous en saisissant un nouveau code MFA généré par votre application (Google Authenticator, Authy, YubiKey...). |
| **Écran étroit / petits écrans** | Débordement d'affichage sur fenêtre réduite. | L'interface s'adapte automatiquement, les boutons restent accessibles et les noms trop longs sont tronqués avec une ellipse. |

---

*Profitez bien de votre expérience avec **Sweetbucket** ! 🍯🍭*
