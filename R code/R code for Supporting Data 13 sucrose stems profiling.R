library(readxl)
library(ggplot2)
library(tidyverse)
library(dplyr)
library(ggpmisc)
library(ggpubr)

#### Sucrose 2022 2month old ####
data = read_excel("Supporting Data 13 sucrose stems profiling.xlsx", sheet = "2022 Suc")

data$Genotype = factor(data$Genotype,levels=c("WT","RNAi60"),
                       labels = c("WT","italic(RNAi60)"))
data$treatment = factor(data$treatment, levels = c('intact','decap')) 
which(is.na(data), arr.ind=TRUE) # None


data$internode = factor(data$internode, levels = c('int4','int8','int12','int16','int20','int24','int28'))

p =  ggplot(data, aes(as.numeric(x=internode), y=sucrose)) +
  geom_boxplot(aes(x=internode, y=sucrose, color=Genotype),alpha=0) +
  geom_jitter(aes(x=internode, y=sucrose, color=Genotype, shape=treatment),size=1, alpha=0.9, width = 0.2) +
  scale_shape_manual(values=c(1, 16))+
  scale_color_manual(values = c(WT = "#1b7837",
                                "italic(RNAi60)" = "#d95f02")) +
  
  facet_grid(treatment~Genotype, labeller = labeller(Genotype = label_parsed)) +
  ylim(10,40) +
  geom_smooth(color ="#636363", formula = y ~ x, method = 'lm',se = FALSE) + 
  stat_cor(method = "pearson") +
  labs(x = "", y = "Sucrose (umol/g FW) yr2022") + theme_light() + 
  theme(axis.title.y = element_text(size=12, face="bold"),
        axis.text.x = element_text(),
        axis.text=element_text(size=10),
        legend.position="none") 
p
ggsave(p,file="pset2022 Sucrose.png", width = 7, height = 4.3)
ggsave(p,file="pset2022 Sucrose.pdf", width = 7, height = 4.3)
