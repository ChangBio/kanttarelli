library(readxl)
library(tidyverse)
library(ggplot2)
library(gridExtra)

line4 <- read_xlsx("Supporting Data 08 RNAi NaPPI phenotyping.xlsx", sheet = "NaPPi 2019")
line4$Genotype <- factor(line4$Genotype, levels=c("WT","RNAi60","RNAi2","kanttarelli"))
parameters <- colnames(line4)[3:5]


library(fs)
for (i in seq_along(parameters))
{
  ColumnSymbol <- sym(parameters[i])
  ColumnString <- parameters[i]
  
  tiff(path_sanitize(paste("NaPPi ", ColumnString, ".tiff")) , width = 3.1, height = 3, units = "in", res = 700)
  
  print(ggplot(data=line4, aes(x=Genotype, y= !!ColumnSymbol, fill=Genotype, group=Genotype, color=Genotype)) +
          geom_boxplot(alpha=0)  +
          geom_jitter(size=0.7, alpha=0.6, width = 0.3) +
          scale_color_manual(values = c(WT = "#1b7837",
                                        RNAi2 = "#e7298a", 
                                        RNAi60 = "#d95f02",
                                        kanttarelli = "#762a83")) +
          ylab(ColumnString) +
          theme_classic() +
          theme(axis.title.x = element_blank(),legend.position = "none"))
  dev.off()
}

