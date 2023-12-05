import numpy as np

# Tasas promedio de "goles" (rendimiento) para Team Secret y OG
tasa_team_secret = 2.5  # Tasa ficticia para Team Secret
tasa_og = 2.2  # Tasa ficticia para OG

# Función para simular un partido utilizando la distribución de Poisson
def simular_partido(tasa_promedio_goles):
    goles = np.random.poisson(tasa_promedio_goles)
    return goles

# Simular tres partidos para Team Secret y OG
goles_team_secret = sum(simular_partido(tasa_team_secret) for _ in range(3))
goles_og = sum(simular_partido(tasa_og) for _ in range(3))

# Determinar al "ganador" basado en el número total de "goles" en los tres partidos
if goles_team_secret > goles_og:
    print(f"Team Secret ganaría con un total de {goles_team_secret} 'goles' frente a {goles_og} 'goles' de OG.")
elif goles_og > goles_team_secret:
    print(f"OG ganaría con un total de {goles_og} 'goles' frente a {goles_team_secret} 'goles' de Team Secret.")
else:
    print(f"Es un empate, ambos equipos anotaron un total de {goles_team_secret} 'goles'.")