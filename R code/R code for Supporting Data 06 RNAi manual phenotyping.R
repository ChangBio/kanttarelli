library(readxl)
library(ggplot2)
library(tidyverse)

RNAi = read_excel("Supporting Data 06 RNAi manual phenotyping.xlsx", sheet = "all data")
colnames(RNAi) <- c("Genotype", "Week", "Plant height (cm)", "1st order branch number", "2nd order branch number", 
                    "Internode number", "Thickness at soil", "Thickness 10cm above soil")

RNAi$Genotype <- factor(RNAi$Genotype, levels=c("WT","RNAi60","RNAi2","kanttarelli"))
RNAi$Thickness.at.10cm.above.soil <- as.numeric(RNAi$Thickness.at.10cm.above.soil)

library(fs)

tempcols <- c(3:8)
parameters <- colnames(RNAi)[tempcols]


for (i in seq_along(parameters))
{
  ColumnSymbol <- sym(parameters[i])
  ColumnString <- parameters[i]
  
  tiff(path_sanitize(paste("RNAi2020linechart", ColumnString, ".tiff")) , width = 4, height = 3, units = "in", res = 800)
  
  WeeksWithValue <- seq(1,31,2)[!seq(1,31,2)%in%c(13,17,21,25,29)]
  
  pd <- position_dodge(width = 0.2)
  print(ggplot(data=RNAi, aes(x=Week, y= !!ColumnSymbol, fill=Genotype, group=Genotype, color=Genotype)) +
          stat_summary(fun = mean, geom = "point", position = pd) +
          stat_summary(fun = mean, geom = "line", aes(group=Genotype), size = 0.7, position = pd) +
          stat_summary(fun.data = mean_se, geom = "errorbar", size = 0.6, width = 0.2, position = pd) +
          ylab(ColumnString) +
          xlab("weeks") +
          scale_x_continuous(breaks=WeeksWithValue) +
          expand_limits(x = 0, y = 0) +
          scale_color_manual( values = c(WT = "#1b7837",
                                         RNAi2 = "#e7298a", 
                                         RNAi60 = "#d95f02",
                                         kanttarelli = "#762a83")) +
          theme_classic())
  dev.off()
}
































