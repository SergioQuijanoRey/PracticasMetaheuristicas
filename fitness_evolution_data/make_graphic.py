import sys
import numpy as np
import matplotlib.pyplot as plt

# Comprobacion de que estamos pasando bien los parametros de entrada
if len(sys.argv) != 2:
    raise Exception("Bad input parameters")

data_file = sys.argv[1]
y_values = np.load(data_file)
x_values = np.arange(0, len(y_values))

plt.xlabel("Iteracion")
plt.ylabel("Valor del fitness")
plt.title("Evolucion del fitness a lo largo de las iteraciones")
plt.plot(x_values, y_values)
plt.show()
