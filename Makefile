slides.html: index.md
	pandoc --self-contained -s -t revealjs $< -o $@
