library("readxl")
library(dplyr)
library(ggplot2)

## 4 lines: WT, RNAi60, RNAi2, kanttarelli
## in hydroponics in Japan.

SL = read_excel("Supporting Data 03 SL quantification time course.xlsx", sheet = "tidy")
SL$Genotype = factor(SL$Genotype, levels = c("WT","RNAi60","RNAi2","kanttarelli"),
                     labels = c("WT","italic(RNAi60)","italic(RNAi2)","italic(kanttarelli)"))
SL$SLs = factor(SL$SLs, levels = c("novel SL", "CLA", "18-OH-CLA"))
SL$pg = as.numeric(SL$pg)

ggplot(SL, aes(x=`Time (day)`, y=pg,group=interaction(`Time (day)`, Genotype))) +
  geom_boxplot(aes(color=Genotype), alpha=0) +
  geom_jitter(aes(color=Genotype), size=1, alpha=0.7, width=0.2) +
  scale_color_manual( values = c(WT = "#1b7837",
                                 "italic(RNAi2)" = "#e7298a", 
                                 "italic(RNAi60)" = "#d95f02",
                                 "italic(kanttarelli)" = "#762a83")) +
  ylim(NA, 700) +
  facet_grid(SLs ~ Genotype, scales = "free",space = "free", labeller = labeller(Genotype = label_parsed)) + 
  labs(x = "Time in hydropnic solution (day)", y = "SLs amount from hydroponic solution (pg)") +
  theme_bw() +
  theme(axis.title = element_text(size=13, face="bold"),
        axis.text.x = element_text(),
        axis.text=element_text(size=13),
        strip.text = element_text(size = 12),
        legend.position="none") 
ggsave(filename="SL time course quantification.png", width = 8, height = 5, unit = "in", device='png', dpi=700)
ggsave(filename="SL time course quantification.pdf", width = 8, height = 5, unit = "in", device='pdf', dpi=700)

