library(readxl)
library(tidyverse)
library(ggplot2)


#### set1 ####

set1 = read_excel("germplasm_resources_sets1and2.xlsx", sheet = "set1")
set1$Genotype = as.factor(set1$Genotype)

parameters = colnames(set1)[c(3,6,7)]
set1$Genotype = factor(set1$Genotype, levels = c("WT","kanttarelli","Pöytäkoivu","Luutakoivu"))
set1$Week = as.integer(set1$Week)

plot_list <- list()

for (i in seq_along(parameters))
{
  
  ColumSymbol = sym(parameters[i])
  ColumnString = parameters[i]
  
  WeeksWithValue = seq(1,12)[!seq(1,12)%in%c(7,9,11)]
  
  pd = position_dodge(width = 0.25)
 
  # Create the ggplot object and add it to the plot_list 
  plot_list[[i]] <- ggplot(data = set1, aes(x = Week, y = !!ColumSymbol, color = Genotype, group = Genotype)) +
          stat_summary(fun.y = mean, geom = "point",position = pd) +
          stat_summary(fun.y = mean, geom = "line",aes(group = Genotype), size = 0.7, position = pd) +
          stat_summary(fun.data = mean_se, geom = "errorbar", size= 0.6, width = 0.2, position = pd) +
          ylab(ColumnString) +
          xlab("Week") +
          scale_x_continuous(breaks = WeeksWithValue) +
          scale_color_manual(values = c(WT = "#1b7837",
                                        kanttarelli = "#762a83",
                                        Pöytäkoivu = "#f781bf",
                                        Luutakoivu = "#377eb8")) +
          theme_classic() +
          theme(legend.position = c(0.2,0.86), legend.title = element_blank())

  # Save each plot as a PDF file
  pdf(paste0("set1_", as.character(parameters[i]), ".pdf"), width = 4, height = 3)
  print(plot_list[[i]])
  dev.off()
}


#### set2 ####

set2 = read_excel("germplasm_resources_sets1and2.xlsx", sheet = "set2")
set2$`Height (cm)` = as.numeric(set2$`Height (cm)`)
set2$Genotype = as.factor(set2$Genotype)

parameters = colnames(set2)[c(3,6,7)]
set2$Genotype = factor(set2$Genotype, levels = c("WT","kanttarelli","E8032 Lutta"))
set2$Week = as.integer(set2$Week)

plot_list2 <- list()

for (i in seq_along(parameters))
{
  
  ColumSymbol = sym(parameters[i])
  ColumnString = parameters[i]
  
  WeeksWithValue = seq(1,12)[!seq(1,12)%in%c(7,9,11)]
  
  pd = position_dodge(width = 0.25)
  
  # Create the ggplot object and add it to the plot_list2 
  plot_list2[[i]] <- ggplot(data = set2, aes(x = Week, y = !!ColumSymbol, color = Genotype, group = Genotype)) +
    stat_summary(fun.y = mean, geom = "point",position = pd) +
    stat_summary(fun.y = mean, geom = "line",aes(group = Genotype), size = 0.7, position = pd) +
    stat_summary(fun.data = mean_se, geom = "errorbar", size= 0.6, width = 0.2, position = pd) +
    ylab(ColumnString) +
    xlab("Week") +
    scale_x_continuous(breaks = WeeksWithValue) +
    scale_color_manual(values = c(WT = "#1b7837",
                                  kanttarelli = "#762a83",
                                  `E8032 Lutta` = "#969696")) +
    theme_classic() +
    theme(legend.position = c(0.2,0.86), legend.title = element_blank())
  
  # Save each plot as a PDF file
  pdf(paste0("set2_", as.character(parameters[i]), ".pdf"), width = 4, height = 3)
  print(plot_list2[[i]])
  dev.off()
}
