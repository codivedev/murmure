# Murmure

Application de dictée vocale utilisant l'API Groq Whisper.

## Fonctionnalités

- Transcription voix-vers-texte via l'API Groq Whisper
- Activation par raccourci clavier global (défaut : Ctrl+Space)
- Icône dans la barre système
- Assistant de configuration pour les nouveaux utilisateurs
- Interface en français
- Support multilingue (Français, Anglais, Espagnol, Allemand, Italien, Portugais, Japonais, Chinois, Coréen)

## Installation (Pop!_OS/Ubuntu)

### Prérequis

- Node.js 18+ et npm
- Rust (dernière version stable)
- Tauri CLI

### Installation

1. Installer les dépendances système :

```bash
sudo apt update
sudo apt install -y libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libxdo-dev \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

2. Installer Rust :

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. Installer Tauri CLI :

```bash
npm install -g @tauri-apps/cli
```

4. Cloner et installer les dépendances :

```bash
git clone https://github.com/codivedev/murmure.git
cd murmure
npm install
```

## Utilisation

### Première utilisation

1. Lancer l'application
2. L'assistant de configuration vous guidera pour :
   - Entrer votre clé API Groq
   - Configurer le raccourci clavier
   - Tester votre microphone

### Enregistrement

1. Appuyez sur le raccourci (défaut : Ctrl+Space)
2. Parlez votre texte
3. Appuyez à nouveau sur le raccourci pour arrêter
4. La transcription s'affiche et est insérée automatiquement

### Barre système

- Clic droit sur l'icône pour accéder aux actions rapides
- Ouvrir les paramètres ou quitter l'application

## Configuration

### Paramètres

Accédez aux paramètres via la barre système ou la fenêtre principale :

- **Clé API** : Votre clé API Groq
- **Raccourci** : Combinaison de touches pour activer l'enregistrement
- **Langue** : Langue de transcription

## Développement

### Commandes

```bash
# Installer les dépendances
npm install

# Serveur de développement (frontend)
npm run dev

# Compiler le frontend
npm run build

# Lancer l'application en mode développement
npm run tauri:dev

# Compiler l'application pour la production
npm run tauri:build
```

### Structure du projet

- `src/` - Code frontend React/TypeScript
- `src-tauri/` - Code backend Rust
- `src/components/` - Composants React
- `src/hooks/` - Hooks React personnalisés
- `src/utils/` - Utilitaires et API Tauri

## Technologies

- **Frontend** : React 19, TypeScript, TailwindCSS, Vite
- **Backend** : Tauri v2, Rust
- **API** : Groq Whisper

## Problèmes connus

- X11 uniquement (Wayland non supporté pour les raccourcis globaux)
- Module audio en cours de développement

## Licence

MIT License - voir le fichier [LICENSE](LICENSE) pour plus de détails.

## Auteur

[CodiveDev](https://github.com/codivedev)

## Liens

- [GitHub](https://github.com/codivedev/murmure)
- [Console Groq](https://console.groq.com)