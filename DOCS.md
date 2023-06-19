# Documentation des intercations websockets

## Première connexion au serveur :
<br/>

- Réponse unicast, envoie de l'état de la partie : 
      
  ```json
      {
    "command": "connection",
    "data": {
        "clients": [
            {
                "id": 1,
                "position": {
                    "x": 0.0,
                    "y": 0.0
                },
                "pseudo": "player1"
            }
        ],
        "game": {
            "duration": 7200,
            "flags": [
                {
                    "action_radius": 8,
                    "id": "64fe8180664711ce068b395f4841af21",
                    "is_captured": false,
                    "player_id": 0,
                    "position": {
                        "x": 7.737285633001739,
                        "y": 48.5286601310684
                    },
                    "team_id": 0,
                    "time": 30,
                    "visibility_radius": 15
                },
                {
                    "action_radius": 8,
                    "id": "4207ceb8d9b0f5220b022068ee2e3ac4",
                    "is_captured": false,
                    "player_id": 0,
                    "position": {
                        "x": 7.734350151369843,
                        "y": 48.52915927791176
                    },
                    "team_id": 0,
                    "time": 30,
                    "visibility_radius": 15
                },
                {
                    "action_radius": 8,
                    "id": "c5df5264e72510f9e4a72cb408065146",
                    "is_captured": false,
                    "player_id": 0,
                    "position": {
                        "x": 7.7391231335825985,
                        "y": 48.52964274796544
                    },
                    "team_id": 0,
                    "time": 30,
                    "visibility_radius": 15
                },
            ],
            "game_mode": "CAPTURE",
            "is_loading": false,
            "items": [
                {
                    "action_radius": 8,
                    "description": "Permet d'observer tous les élements de la carte pendant une durée défini.",
                    "effect_duration": 30,
                    "id": "9f78b39741dc29fb00bfa859e03ea70f",
                    "item_type": 0,
                    "name": "Satellite",
                    "position": {
                        "x": 7.736928253581617,
                        "y": 48.52917450594368
                    },
                    "visibility_radius": 15
                },
                {
                    "action_radius": 8,
                    "description": "Permet d'observer tous les élements de la carte pendant une durée défini.",
                    "effect_duration": 30,
                    "id": "907f05fff0b4fcae05b802065e2974a6",
                    "item_type": 0,
                    "name": "Satellite",
                    "position": {
                        "x": 7.739856602185479,
                        "y": 48.530750005926194
                    },
                    "visibility_radius": 15
                },
            ],
            "map": "{geojson-map-data}",
            "teams": {
                "71": {
                    "color": "#f05656",
                    "id": 71,
                    "name": "L'ISS",
                    "nb_players": 0,
                    "players": [],
                    "score": 0
                },
                "72": {
                    "color": "#61f5a1",
                    "id": 72,
                    "name": "Les Astronautes",
                    "nb_players": 0,
                    "players": [],
                    "score": 0
                },
                "73": {
                    "color": "#5c5cf0",
                    "id": 73,
                    "name": "Apollo 8",
                    "nb_players": 5,
                    "players": [],
                    "score": 0
                },
                "74": {
                    "color": "#ca4e9d",
                    "id": 74,
                    "name": "Apollo 9",
                    "nb_players": 5,
                    "players": [],
                    "score": 0
                }
            },
            "traps": []
        }
    }
  ```

- Réponse broadcast :

  ```json
    {
        "command": "new-player",
        "data": {
            "client_id": 1,
            "pseudo": "pseudo"
        }
    }
    ```

<br/> <br/>

## Commandes :

<br/>

### Charger la map au format geojson :

<br/>

- Requêtes :

```json
{
    "command": "load-map",
    "data": {}
}
```

- Réponse unicast :

```json
{
  "command": "load-map",
  "data": "{geojson-map-data}"
}
```

<br/>

### Envoi d'un message dans le chat :

<br/>

- Requêtes :

```json
{
    "command": "chat",
    "data": {
        "msg": "hello players"
    }
}
```

- Réponse broadcast :

```json
{
  "command": "chat",
  "data": {
    "pseudo": "pseudo",
    "msg": "hello players"
  }
}
```
<br/>

### mise à jour de la position : 

<br/>

- Requêtes :

```json
{
    "command": "update-position",
    "data": {
        "position":
        {
            "x": 23.32980128932713,
            "y": 12.10938271927393
        }
    }
}
```

- Réponse broadcast :

```json
{
    "command": "update-position",
    "data": {
        "client_id": 1,
        "position": {
            "x": 23.32980128932713,
            "y": 12.10938271927393
        },
        "pseudo": "player1"
    }
}
```
<br/>

Après chaque mise à jour de position on calcul le zone de visibilité et d'action de tous les points :

```json
- Réponse
{
    "command": "check-coords",
    "data": {
        "player_zone": [],
        "points_zone": [
            {
                "capturable": false,
                "point_id": "35610b4e5da9f63630efafc3b468b8d6",
                "visible": false
            },
            {
                "capturable": false,
                "point_id": "449374f90f01d2915ebd3149a23ed44c",
                "visible": false
            },
            {
                "capturable": false,
                "point_id": "13c74e7f3ced45b5d845dedd0a738e61",
                "visible": false
            },
            {
                "capturable": false,
                "point_id": "8933d1e2d33657cf2c418c29dcd07bbc",
                "visible": false
            },
            {
                "capturable": false,
                "point_id": "d493cbaf6cc7757e72979cb36b715622",
                "visible": false
            },
            {
                "capturable": false,
                "point_id": "542acbd4c8e1c93478a1ac8331a21d39",
                "visible": false
            },
            {
                "capturable": false,
                "point_id": "1cff76df99d6812be9e3d67237590365",
                "visible": false
            },
            {
                "capturable": false,
                "point_id": "3ae3abb01e9fe7b554aa65fdf2f54032",
                "visible": false
            },
            {
                "capturable": false,
                "point_id": "346d1c7043745b7113b066b915a768ab",
                "visible": false
            },
            {
                "capturable": false,
                "point_id": "66590c17a7bccc38391b73d17aa351b5",
                "visible": false
            },
            {
                "capturable": false,
                "point_id": "a456a5bfbd1abd57c689fc67d0cac369",
                "visible": false
            },
            {
                "capturable": false,
                "point_id": "743d4d7291c93fc794cb8902f87cef80",
                "visible": false
            },
            {
                "capturable": false,
                "point_id": "9ab89c03ef6dcf423a95d38702b92536",
                "visible": false
            },
            {
                "capturable": false,
                "point_id": "c4283b381a05939b454946c883f08859",
                "visible": false
            },
            {
                "capturable": false,
                "point_id": "9f78b39741dc29fb00bfa859e03ea70f",
                "visible": false
            },
            {
                "capturable": false,
                "point_id": "907f05fff0b4fcae05b802065e2974a6",
                "visible": false
            },
            {
                "capturable": false,
                "point_id": "324360494bba7d6b842fdf2c529bb485",
                "visible": false
            },
            {
                "capturable": false,
                "point_id": "cfedbd4c256d7e3cd4505a088463ac34",
                "visible": false
            },
            {
                "capturable": false,
                "point_id": "27f0b984efd519b41cb2f1178ae94cb3",
                "visible": false
            },
            {
                "capturable": false,
                "point_id": "62d426c002e3f169c296658e02123bc5",
                "visible": false
            },
            {
                "capturable": false,
                "point_id": "8730b4ff0f0d18dc5a3ab90cf82bcc0b",
                "visible": false
            },
        ],
        "trap_zone": []
    }
}
```

### Rejoindre une équipe :

<br/>

- Requêtes :

```json
{
    "command": "join-team",
    "data": {
        "team_id": 1
    }
}
```

- Réponse broadcast :

```json
{
  "command": "join-team",
  "data": {
    "message": "The player {pseudo} has joined a team",
    "team_id": 1,
    "player_id": 1,
    "pseudo": "pmayer1"
      }
}
```

### Lancer une partie :

<br/>

- Requêtes :

```json
{
    "command":"game-launcher",
    "data": {
    "trigger":true
    }
}
```

- Réponse broadcast :

```json
{
    "command": "game-launcher",
    "data": {
        "game_loading": true,
        "message": "The game begin..."
    }
}
```

### Arrêter une partie :

<br/>

- Requêtes :

```json
{
    "command":"game-launcher",
    "data": {
    "trigger":false
    }
}
```

- Réponse broadcast :

```json
{
    "command": "game-launcher",
    "data": {
        "game_loading": false,
        "message": "The game is over"
    }
}
```
<br>

### Capturer un drapeau :

<br/>

- Requêtes :

```json
{
    "command":"capture-flag",
    "data":
    {
        "flag_id":"85c3d747da3f0c84868ac038fe02702f"
    }  
}
```

- Réponse broadcast :

```json
{
  "command": "capture-flag",
  "data": {
    "flag_id": "85c3d747da3f0c84868ac038fe02702f",
    "is_capture": true,
    "player": {
        "id": 1,
        "pseudo": "player1",
        "score": 1
    },
    "team": {
        "color": "string",
        "id": 1,
        "nb_players": 4,
        "score": 2,
        "players": [
            {
                "id": 1,
                "pseudo": "player1"
            },
            {
                "id": 1,
                "pseudo": "player2"
            }
        ]
    }
}
}
```

- Réponse broadcast lorsqu'un drapeau redevient neutre :

```json
{
    "command": "free-flag",
    "data": {
        "flag_id": "85c3d747da3f0c84868ac038fe02702f",
        "is_capture": false
    }
}
```
<br>

### Prendre un item pour le mettre dans l'inventaire :
<br>

- Requête :
```json
{
    "command": "get-item",
    "data": {
        "item_id": "0e4d486c60b5d4d0ae3d48fc9fead31f"
    }
}
```

Réponse broadcast : 
```json
{
    "command": "get-item",
    "data": {
        "item_id": "0e4d486c60b5d4d0ae3d48fc9fead31f"
    }
}
```

### Command pour utiliser un item de son inventaire :
- Requête :
```json
{
    "command": "use-item",
    "data": {
        "item_id": "d3d99f3e23e1494a5160c14eaeaa7454",
        "position": {
                        "x": 7.739439368093798,
                        "y": 48.53065636477427
                    }
    }
}
```

- Réponse broadcast quand on envoi un satellite :
```json
{
    "command": "use-item",
    "data": {
        "client_id": 1,
        "enable": true,
        "time": 30,
        "type_id": 0
    }
}
```

- Réponse broadcast quand on pose une mine

```json
    {
    "command": "use-item",
    "data": {
        "enable": true,
        "item": {
            "action_radius": 8,
            "description": "Once placed, if a player from the opposing team steps on it, he can no longer retrieve a flag or item for a set period of time.",
            "effect_duration": 30,
            "id": "fb656a7d4dea8106ad982c07ddf2f7fc",
            "item_type": 2,
            "name": "Mine",
            "position": {
                "x": 7.739439368093798,
                "y": 48.53065636477427
            },
            "visibility_radius": 15
        },
        "position": {
            "x": 7.739439368093798,
            "y": 48.53065636477427
        },
        "team_id": 0,
        "time": 30,
        "type_id": 2
    }
}
```

- Réponse broadcast quand on prend un backpack :
```json
{
    "command": "use-item",
    "data": {
        "additionnal_location": 2,
        "enable": true,
        "type_id": 1
    }
}
```

- Réponse broadcast quand on envoi un brouilleur :
```json
{
    "command": "use-item",
    "data": {
        "enable": true,
        "team_id": 117,
        "time": 30,
        "type_id": 3
    }
}
```

- Réponse broadcast quand l'effet du satellite est terminé :

```json
{
    "command": "item-effect-finished",
    "data": {
        "client_id": 1,
        "enable": false,
        "time": 0,
        "type_id": 0
    }
}
```

- Réponse broadcast quand l'effet du brouilleur est terminé :
```json
{
    "command": "item-effect-finished",
    "data": {
        "enable": false,
        "team_id": 117,
        "time": 0,
        "type_id": 3
    }
}
```
<br>

### Commande pour faire exploser une mine :

<br>

- Requête :
```json
{
    "command": "mine-explode",
    "data": {
        "mine_id": "fb656a7d4dea8106ad982c07ddf2f7fc"
    }
}
```

- Réponse broadcast :

```json
{
    "command": "mine-explode",
    "data": {
        "mine_id": "fb656a7d4dea8106ad982c07ddf2f7fc"
    }
}
```

- Réponse broadcast quand le malus de l'explosion est fini :

```json
{
    "command": "item-effect-finished",
    "data": {
        "enable": false,
        "item": {
            "action_radius": 8,
            "description": "Once placed, if a player from the opposing team steps on it, he can no longer retrieve a flag or item for a set period of time.",
            "effect_duration": 93,
            "id": "fb656a7d4dea8106ad982c07ddf2f7fc",
            "item_type": 2,
            "name": "Mine",
            "position": {
                "x": 7.739439368093798,
                "y": 48.53065636477427
            },
            "visibility_radius": 15
        },
        "position": {
            "x": 7.739439368093798,
            "y": 48.53065636477427
        },
        "team_id": 117,
        "time": 0,
        "type_id": 2
    }
}
```

<br>

### Commande pour ajouter un point d'interêt :

- Requête :

```json
{
    "command": "set-point-interest",
    "data": {
        "point_id": "djf8UHSB7D7SHJ90",
        "position": {
            "x": 2.302929102939,
            "y": 3.939090349049
        }
    }
}
```
- Réponse broadcast :

```json
{
    "command": "set-point-interest",
    "data": {
        "player_id": 2,
        "point_id": "djf8UHSB7D7SHJ90",
        "position": {
            "x": 2.302929102939,
            "y": 3.939090349049
        },
        "team_id": 117
    }
}
```

### Commande pour ajouter un drapeau (rôle maitre du jeu) :

- Requête :
```json
{
    "command": "add-flag",
    "data": {
        "flag": {
            "action_radius": 8,
            "id": "06ecd56da9ada181eea8a81857aeb6e5",
            "is_captured": false,
            "player_id": 0,
            "position": {
                "x": 7.737716810999951,
                "y": 48.53058482610307
            },
            "team_id": 0,
            "time": 30,
            "visibility_radius": 15
        }
    }
}

- Réponse broadcast :

```json
{
    "command": "add-flag",
    "data": {
        "flag": {
            "action_radius": 8,
            "id": "06ecd56da9ada1dddd81eea8a81857aeb6e5",
            "is_captured": false,
            "player_id": 0,
            "position": {
                "x": 7.737716810999951,
                "y": 48.53058482610307
            },
            "team_id": 0,
            "time": 30,
            "visibility_radius": 15
        }
    }
}
```

### Commande pour supprimer un drapeau (rôle maitre du jeu) :

- Requête :
```json
{
    "command": "remove-flag",
    "data": {
        "flag_id": "06ecd56da9ada1dddd81eea8a81857aeb6e5"
    }
}
```

- Réponse broadcast :
```json
{
    "command": "remove-flag",
    "data": {
        "flag_id": "06ecd56da9ada1dddd81eea8a81857aeb6e5"
    }
}
```

### Commande pour ajouter un item (rôle maitre du jeu) :

- Requête :
```json
{
    "command": "add-item",
    "data": {
        "item":   {
            "action_radius": 8,
            "description": "Allows you to observe all elements of the map for a set period of time.",
            "effect_duration": 30,
            "id": "c0bfa36774296b1c26e8dkkk0b17b37d265",
             "item_type": 0,
            "name": "Satellite",
            "position": {
                "x": 7.734315656413592,
                "y": 48.530623494800345
             },
            "visibility_radius": 15
        }
    }
}
```

- Réponse broadcast :

```json
{
    "command": "add-item",
    "data": {
        "item": {
            "action_radius": 8,
            "description": "Allows you to observe all elements of the map for a set period of time.",
            "effect_duration": 30,
            "id": "c0bfa36774296b1c26e8dkkk0b17b37d265",
            "item_type": 0,
            "name": "Satellite",
            "position": {
                "x": 7.734315656413592,
                "y": 48.530623494800345
            },
            "visibility_radius": 15
        }
    }
}
```

### Commande pour supprimer un item (rôle maitre du jeu):

- requête :
```json
{
    "command": "remove-item",
    "data": {
        "item_id": "c0bfa36774296b1c26e8dkkk0b17b37d265"
    }
}
```

- réponse broadcast :
```json
{
    "command": "remove-item",
    "data": {
        "item_id": "c0bfa36774296b1c26e8dkkk0b17b37d265"
    }
}
```