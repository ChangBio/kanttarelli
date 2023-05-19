library(readxl)
library(tidyverse)
library(ggplot2)

#### set1 ####

set1 = read_excel("Supporting Data 01 germplasm resources 3 sets.xlsx", sheet = "set1")
set1$Genotype = gsub("Poutapilvi","Cloud birch",set1$Genotype)
set1$Genotype = as.factor(set1$Genotype)

parameters = colnames(set1)[c(3,6,7)]
set1$Genotype = factor(set1$Genotype, levels = c("WT","kanttarelli","Cloud birch"))
set1$Week = as.integer(set1$Week)

for (i in seq_along(parameters))
{
  png(paste("Set1", as.character(parameters[i]), ".png"), width=4,height=3,units="in",res=1200)
  
  ColumSymbol = sym(parameters[i])
  ColumnString = parameters[i]
  
  WeeksWithValue = seq(1,12)[!seq(1,12)%in%c(7,9,11)]
  
  pd = position_dodge(width = 0.25)
  print(ggplot(data = set1, aes(x = Week, y = !!ColumSymbol, color = Genotype, group = Genotype)) +
          stat_summary(fun.y = mean, geom = "point",position = pd) +
          stat_summary(fun.y = mean, geom = "line",aes(group = Genotype), size = 0.7, position = pd) +
          stat_summary(fun.data = mean_se, geom = "errorbar", size= 0.6, width = 0.2, position = pd) +
          ylab(ColumnString) +
          xlab("Week") +
          scale_x_continuous(breaks = WeeksWithValue) +
          scale_color_manual(values = c(WT = "#1b7837",
                                        kanttarelli = "#762a83",
                                        `Cloud birch` = "#ff7f00")) +
          theme_classic() +
          theme(legend.position = c(0.2,0.86), legend.title = element_blank()))
  
  dev.off()
}


#### set2 ####

set2 = read_excel("Supporting Data 01 germplasm resources 3 sets.xlsx", sheet = "set2")
set2$Genotype = gsub("Pöytäkoivu","Table birch",set2$Genotype)
set2$Genotype = as.factor(set2$Genotype)

parameters = colnames(set2)[c(3,6,7)]
set2$Genotype = factor(set2$Genotype, levels = c("WT","kanttarelli","Table birch","Luutakoivu"))
set2$Week = as.integer(set2$Week)

for (i in seq_along(parameters))
{
  png(paste("Set2", as.character(parameters[i]), ".png"), width=4,height=3,units="in",res=1200)
  
  ColumSymbol = sym(parameters[i])
  ColumnString = parameters[i]
  
  WeeksWithValue = seq(1,12)[!seq(1,12)%in%c(7,9,11)]
  
  pd = position_dodge(width = 0.25)
  print(ggplot(data = set2, aes(x = Week, y = !!ColumSymbol, color = Genotype, group = Genotype)) +
          stat_summary(fun.y = mean, geom = "point",position = pd) +
          stat_summary(fun.y = mean, geom = "line",aes(group = Genotype), size = 0.7, position = pd) +
          stat_summary(fun.data = mean_se, geom = "errorbar", size= 0.6, width = 0.2, position = pd) +
          ylab(ColumnString) +
          xlab("Week") +
          scale_x_continuous(breaks = WeeksWithValue) +
          scale_color_manual(values = c(WT = "#1b7837",
                                        kanttarelli = "#762a83",
                                        `Table birch` = "#f781bf",
                                        Luutakoivu = "#377eb8")) +
          theme_classic() +
          theme(legend.position = c(0.2,0.86), legend.title = element_blank()))
  
  dev.off()
}

#### set3 ####

set3 = read_excel("Supporting Data 01 germplasm resources 3 sets.xlsx", sheet = "set3")
set3$`Height (cm)` = as.numeric(set3$`Height (cm)`)
set3$Genotype = as.factor(set3$Genotype)

parameters = colnames(set3)[c(3,6,7)]
set3$Genotype = factor(set3$Genotype, levels = c("WT","kanttarelli","E8032 Lutta","Peera 6", "Peera 16","Peera 28"))
set3$Week = as.integer(set3$Week)

for (i in seq_along(parameters))
{
  png(paste("Set3", as.character(parameters[i]), ".png"), width=5.3,height=3,units="in",res=1200)
  
  ColumSymbol = sym(parameters[i])
  ColumnString = parameters[i]
  
  WeeksWithValue = seq(1,12)[!seq(1,12)%in%c(7,9,11)]
  
  pd = position_dodge(width = 0.25)
  print(ggplot(data = set3, aes(x = Week, y = !!ColumSymbol, color = Genotype, group = Genotype)) +
          stat_summary(fun.y = mean, geom = "point",position = pd) +
          stat_summary(fun.y = mean, geom = "line",aes(group = Genotype), size = 0.7, position = pd) +
          stat_summary(fun.data = mean_se, geom = "errorbar", size= 0.6, width = 0.2, position = pd) +
          ylab(ColumnString) +
          xlab("Week") +
          scale_x_continuous(breaks = WeeksWithValue) +
          scale_color_manual(values = c(WT = "#1b7837",
                                        kanttarelli = "#762a83",
                                        `E8032 Lutta` = "#969696",
                                        `Peera 6` = "#a65628",
                                        `Peera 16` = "gold1",
                                        `Peera 28` = "#e41a1c")) +
          theme_classic() +
          theme(legend.title = element_blank()))
  
  dev.off()
}


