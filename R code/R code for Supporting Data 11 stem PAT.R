library(readxl)
library(ggplot2)
library(tidyverse)
library(dplyr)
library(ggpmisc)
library(ggpubr)

after <- read_excel("Supporting Data 11 stem PAT.xlsx")
after$CPM1 <- as.numeric(after$CPM1)
colnames(after)[5] <- "CPM"
after$Genotype = gsub("NegtiveControl","NC",after$Genotype)

after$Genotype = factor(after$Genotype,levels=c("WT","RNAi60","kanttarelli","NC"),
                        labels = c("WT","italic(RNAi60)","italic(kanttarelli)", "NC"))

after$Internode = factor(after$Internode, levels = c('int3','int5','int7','int9','int11',"NC"))


PAT = ggplot(after, aes(as.numeric(x=Internode), y=CPM)) +
  geom_boxplot(aes(x=Internode, y=CPM, color=Genotype),alpha=0) +
  geom_jitter(aes(x=Internode, y=CPM, color=Genotype),size=1, alpha=0.9, width = 0.3) +
  scale_color_manual(values = c(WT = "#1b7837",
                                "italic(RNAi60)" = "#d95f02",
                                "italic(kanttarelli)" = "#762a83",
                                NC = "#363434")) +
  facet_grid(~Genotype, scales = "free",space = "free", labeller = labeller(Genotype = label_parsed)) +
  geom_smooth(color ="#636363", formula = y ~ x, method = 'lm',se = FALSE) + 
  stat_cor(method = "pearson") +
  ylim(NA, 500) +
  labs(x = "", y = "Auxin transported (CPM)") + theme_light() + 
  theme(axis.title.y = element_text(size=16, face="bold"),
        axis.text.x = element_text(),
        axis.text=element_text(size=13),
        strip.text = element_text(size = 13),
        legend.position="none") 
PAT
ggsave(filename="PAT_reanalysis2022_PearsonCorrelation.png", width = 8, height = 3.5, unit = "in", device='png', dpi=700)



