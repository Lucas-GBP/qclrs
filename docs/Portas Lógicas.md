## Portas Lógicas em forma matricial
$$H = \frac{1}{\sqrt{2}}\begin{bmatrix}
	1 & 1\\
	1 & -1
\end{bmatrix}$$
$$S = R_z(\pi/2) = \begin{bmatrix}
	1 & 0\\
	0 & i
\end{bmatrix}$$
$$CNOT = \begin{bmatrix}
	1 & 0 & 0 & 0\\
	0 & 1 & 0 & 0\\
	0 & 0 & 0 & 1\\
	0 & 0 & 1 & 0\\
\end{bmatrix}$$
## Portas derivadas
$$Z = SS$$
$$X = HZH = HSSH$$
$$S^\dagger = ZS = SSS$$
$$Y = SXS^\dagger = SHSSHSSS$$