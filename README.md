# Le saviez-vous ?
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/AdrienGras/le-saviez-vous-generator/Rust?style=for-the-badge)

Le saviez-vous est une mine de savoir inépuisable et surtout fausse 😂

C'est le support d'un PoC pour tester la création d'apps web via Rust, Rocket et Tera.

## Installation et lancement

Vous devez avoir docker d'installé sur votre machine.

```bash
git clone https://github.com/AdrienGras/le-saviez-vous-generator
cd le-saviez-vous-generator
cp .env_template .env
# modifiez les variables de ROCKET si nécessaire
docker-compose up --build -d
```

Vous pouvez vous rendre sur `http://localhost:3002` et admirer le résultat 🚀.