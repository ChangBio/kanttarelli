library(readxl)
library(tidyverse)
library(ggplot2)

# Scannned at week 45
# 2020 12 21

raw = read_excel("Supporting Data 07 RNAi greenhouse scannning.xlsx", sheet = "after dormancy")
RNAi = raw %>% 
  mutate_all(~as.numeric(as.character(.))) 
RNAi$Genotype = raw$Genotype

apply(RNAi, 2, function(x) any(is.na(x))) # no NAs

RNAi$Genotype = factor(raw$Genotype,levels = c("WT","RNAi60","RNAi2","kanttarelli"))

# 172 degree, 8cm 1st order branch, RNAi60
# 164 degree, 7cm 1st order branch, Kanttarelli
# 135 degree, 2cm 1st order branch; WT
# remove them, not possible and not observed, technical errors

first.order = RNAi %>% filter(Order==1)
first.order = first.order[!(first.order$BaseTipAngle>135),]

#### 0 ####
##### check IDs of the scanned trees in the winter 2020 ####
# WT:  2 5 7 8 9 10 11 15
WT=RNAi %>% filter(Genotype=="WT")
sort(unique(WT$TreeID))
# R2:  5 8 9 10	11 13	15
R2=RNAi %>% filter(Genotype=="RNAi2")
sort(unique(R2$TreeID))
# R60: 1 4 5 6 7 10 11
R60=RNAi %>% filter(Genotype=="RNAi60") 
sort(unique(R60$TreeID))
# Kan: 2 5 8 9 10 12 13
kan=RNAi %>% filter(Genotype=="kanttarelli")
sort(unique(kan$TreeID))


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
ggsave(BaseTipAngle, file="RNAi2020winter_1st_branch_BaseTipAngle.png", width = 4, height = 3)
ggsave(BaseTipAngle, file="RNAi2020winter_1st_branch_BaseTipAngle.pdf", width = 4, height = 3)

BaseTipAngle.wt  <- RNAi %>% filter(Genotype == "WT") %>% select(BaseTipAngle)
BaseTipAngle.r60 <- RNAi %>% filter(Genotype == "RNAi60") %>% select(BaseTipAngle)
BaseTipAngle.r2  <- RNAi %>% filter(Genotype == "RNAi2") %>% select(BaseTipAngle)

t.test(BaseTipAngle.r60, BaseTipAngle.wt) # p-value = 4.995e-08  WT & RNAi60 are different
t.test(BaseTipAngle.r2, BaseTipAngle.wt)  # p-value = 9.088e-15  WT & RNAi2 are different


