library(readxl)
library(ggplot2)
library(tidyverse)
library(dplyr)
library(ggpmisc)
library(ggpubr)

#### IAA 2021 2month old ####
set2021 = read_excel("Supporting Data 12 IAA stems profiling.xlsx", sheet = "2021 IAA")

set2021$Genotype = factor(set2021$Genotype,levels=c("WT","RNAi60","kanttarelli"),
                          labels = c("WT","italic(RNAi60)","italic(kanttarelli)"))
set2021$IAA = as.numeric(set2021$IAA)
set2021$internode = factor(set2021$internode, levels = c('int3','int7','int11','int15','int19','int23','int27'))

pset2021 =  ggplot(set2021, aes(as.numeric(x=internode), y=IAA)) +
  geom_boxplot(aes(x=internode, y=IAA, color=Genotype),alpha=0) +
  geom_jitter(aes(x=internode, y=IAA, color=Genotype, shape=treatment),size=1, alpha=0.9, width = 0.2) +
  scale_shape_manual(values=c(1, 16))+
  scale_color_manual(values = c(WT = "#1b7837",
                                "italic(RNAi60)" = "#d95f02",
                                "italic(kanttarelli)" = "#762a83")) +
  
  facet_grid(treatment~Genotype, labeller = labeller(Genotype = label_parsed)) +
  geom_smooth(color ="#636363", formula = y ~ x, method = 'lm',se = FALSE) + 
  stat_cor(method = "pearson") +
  labs(x = "", y = "IAA (pmol/g FW) yr2021") + theme_light() + 
  theme(axis.title.y = element_text(size=12, face="bold"),
        axis.text.x = element_text(),
        axis.text=element_text(size=10),
        legend.position="none") 
pset2021
ggsave(pset2021,file="pset2021 IAA.png", width = 7, height = 2.5)
ggsave(pset2021,file="pset2021 IAA.pdf", width = 7, height = 2.5)


#### IAA 2022 2month old ####
set2022 = read_excel("Supporting Data 12 IAA stems profiling.xlsx", sheet = "2022 IAA")

set2022$Genotype = factor(set2022$Genotype,levels=c("WT","RNAi60"),
                          labels = c("WT","italic(RNAi60)"))
set2022$treatment = factor(set2022$treatment, levels = c('intact','decap')) 
set2022$IAA = as.numeric(set2022$IAA)

set2022$internode = factor(set2022$internode, levels = c('int3','int7','int11','int15','int19','int23','int27'))

pset2022 =  ggplot(set2022, aes(as.numeric(x=internode), y=IAA)) +
  geom_boxplot(aes(x=internode, y=IAA, color=Genotype),alpha=0) +
  geom_jitter(aes(x=internode, y=IAA, color=Genotype, shape=treatment),size=1, alpha=0.9, width = 0.2) +
  scale_shape_manual(values=c(1, 16))+
  scale_color_manual(values = c(WT = "#1b7837",
                                "italic(RNAi60)" = "#d95f02")) +
  
  facet_grid(treatment~Genotype, labeller = labeller(Genotype = label_parsed)) +
  geom_smooth(color ="#636363", formula = y ~ x, method = 'lm',se = FALSE) + 
  stat_cor(method = "pearson") +
  labs(x = "", y = "IAA (pmol/g FW) yr2022") + theme_light() + 
  theme(axis.title.y = element_text(size=12, face="bold"),
        axis.text.x = element_text(),
        axis.text=element_text(size=10),
        legend.position="none") 
pset2022
ggsave(pset2022,file="pset2022 IAA.png", width = 7, height = 4.3)
ggsave(pset2022,file="pset2022 IAA.pdf", width = 7, height = 4.3)
