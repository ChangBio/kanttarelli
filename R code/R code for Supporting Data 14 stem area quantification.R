library("readxl")
library(dplyr)
library(ggplot2)

stem.area <- read_excel("Supporting Data 14 stem area quantification.xlsx", sheet = "R")
stem.area$genotype = factor(stem.area$genotype, levels = c("WT","RNAi60"),
                            labels = c("WT","italic(RNAi60)"))

orders = factor(stem.area$lable,levels=c("int3","int7","int11"))

stem.area.p = ggplot(stem.area, aes(x=orders, y=Area)) +
  geom_boxplot(aes(color=genotype),alpha=0) +
  geom_jitter(aes(x=lable, y=Area, color=genotype),size=1, alpha=0.9, width = 0.3) +
  scale_color_manual(values = c(WT = "#1b7837",
                                "italic(RNAi60)" = "#d95f02")) +
  facet_grid(. ~ genotype, labeller = labeller(genotype = label_parsed)) +


  labs(x = "", y = "Stem area (um2)") + theme_light() + 
  theme(axis.title.y = element_text(size=14, face="bold"),
        axis.text.x = element_text(),
        axis.text=element_text(size=13),
        strip.text = element_text(size = 13),
        legend.position="none") 
stem.area.p
ggsave(filename="PAT_stem_area.png", width = 8, height = 4.7, unit = "in", device='png', dpi=700)
