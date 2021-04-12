all:
	# Tenemos que ejecutar esta secuencia para que se cargue correctamente la bibliografia
	pdflatex Memoria.tex -interaction=nonstopmode
	bibtex Memoria
	pdflatex Memoria.tex -interaction=nonstopmode
	pdflatex Memoria.tex -interaction=nonstopmode
clean:
	@echo "Borrando archivos auxiliares"
	@rm -rf *.aux *.log *.toc *.bbl *.blg
