library(readxl)
library(tidyverse)
library(ggplot2)

# Scannned at week 14
# 2020 May 14

raw=read_excel("Supporting Data 07 RNAi greenhouse scannning.xlsx", sheet = "week14")
raw$Genotype=factor(raw$Genotype,levels = c("WT","RNAi60","RNAi2","kanttarelli"))
# 145 degree, 2cm 1st order branch, RNAi60
# 135 degree, 4cm 1st order branch; Kanttarelli
# remove them, not possible and not observed, technical errors
first.order = raw %>% filter(Order==1)
first.order = first.order[!(first.order$BaseTipAngle>120),]

#### 0 ####
##### check IDs of the scanned trees in the summer 2020 ####
# WT:  14 13 12 06 04 03 01
# R60: 15 14 13 12 09 08 03 02
# R2:   14 12 07 06 04 03 02 01
# Kan: 15 14 11 07 06 04 03 01
# check tree IDs as below
WT=raw %>% filter(Genotype=="WT")
unique(WT$TreeID)

R2=raw %>% filter(Genotype=="RNAi2")
unique(R2$TreeID)

R60=raw %>% filter(Genotype=="RNAi60")
unique(R60$TreeID)

Kan=raw %>% filter(Genotype=="kanttarelli")
unique(Kan$TreeID)

#### robust BaseTipAngle: Branching angle (deg) defined between the parent and the line from the branch base to its tip ####

BaseTipAngle = ggplot(data=first.order, aes(x=Genotype,y=BaseTipAngle, color=Genotype)) +
                geom_boxplot(alpha=0)  +
                geom_jitter(size=0.7, alpha=0.6, width = 0.3) +
                scale_color_manual(values = c(WT = "#1b7837",
                                             RNAi2 = "#e7298a", 
                                             RNAi60 = "#d95f02",
                                             kanttarelli = "#762a83")) +
                labs(title="",x="", y = "1st-order branch angle") +
                theme_classic() +
                theme(legend.position = "none")
ggsave(BaseTipAngle, file="RNAi2020summer_1st_branch_BaseTipAngle.png", width = 4, height = 3)
ggsave(BaseTipAngle, file="RNAi2020summer_1st_branch_BaseTipAngle.pdf", width = 4, height = 3)

BaseTipAngle.wt  <- first.order %>% filter(Genotype == "WT") %>% select(BaseTipAngle)
BaseTipAngle.r60 <- first.order %>% filter(Genotype == "RNAi60") %>% select(BaseTipAngle)
BaseTipAngle.r2  <- first.order %>% filter(Genotype == "RNAi2") %>% select(BaseTipAngle)

t.test(BaseTipAngle.r60, BaseTipAngle.wt) # 2.2e-16  WT & RNAi60 are different
t.test(BaseTipAngle.r2, BaseTipAngle.wt)  # 2.2e-16    WT & RNAi2 are different


  
