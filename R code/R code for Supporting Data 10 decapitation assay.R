library(readxl)
library(ggplot2)
library(tidyverse)
library(dplyr)


after <- read_excel("Supporting Data 10 decapitation assay.xlsx")

after$Treatment = factor(after$Treatment, levels = c("intact", "stem_d", "branch_d", "both_d"))
after$Genotype = factor(after$Genotype, levels = c("WT", "RNAi60"),
                        labels = c("WT","italic(RNAi60)"))

# 
# after.decap.2ndbranching = ggplot(after, aes(x=Treatment, y=bran_2nd, color=Genotype)) +
#   geom_boxplot(alpha=0) +
#   geom_jitter(size=1, alpha=0.9, width = 0.2) +
#   scale_color_manual(values = c(WT = "#1b7837",
#                                 RNAi60 = "#d95f02",
#                                 RNAi2 = "#e7298a",
#                                 Kanttarelli = "#762a83")) +
#   facet_wrap(~Genotype) +
#   labs(x = "", y = "2nd branch numbers") + theme_light() + 
#   theme(axis.title.y = element_text(size=12, face="bold"),
#         axis.text.x = element_text(angle = 45, hjust=0.8),
#         axis.text=element_text(size=10),
#         legend.position="none")
# 


after.decap.2ndbranching.bottom = ggplot(after, aes(x=Treatment, y=`2nd order branches`, color=Genotype)) +
  geom_boxplot(alpha=0) +
  geom_jitter(size=1, alpha=0.9, width = 0.2) +
  scale_color_manual(values = c(WT = "#1b7837",
                                "italic(RNAi60)" = "#d95f02")) +
  ylim(NA,15) +
  facet_wrap(~Genotype,labeller = labeller(Genotype = label_parsed)) +
  labs(x = "", y = "2nd order branch numbers") + theme_light() + 
  theme(axis.title.y = element_text(size=10, face="bold"),
        axis.text.x = element_text(angle = 30, hjust=0.8),
        axis.text=element_text(size=8),
        legend.position="none")
ggsave(filename="after_decap_facet_2nd_bottom7_2020.png", width = 4.5, height = 2.5, device='png', dpi=700)


