library("readxl")
library(dplyr)
library(ggplot2)

## 4 lines: WT, RNAi60, RNAi2, kanttarelli

## 27 days in hydroponics in Japan.

day27WT <- read_excel("Supporting Data 04 O.minor germination assay.xlsx", sheet = "WT_27")
day27kan <- read_excel("Supporting Data 04 O.minor germination assay.xlsx", sheet = "kanttarelli_27")
day27R60 <- read_excel("Supporting Data 04 O.minor germination assay.xlsx", sheet = "RNAi60_27")
day27R2 <- read_excel("Supporting Data 04 O.minor germination assay.xlsx", sheet = "RNAi2_27")

day27 = rbind(day27WT, day27kan, day27R60, day27R2)
day27$Genotype = factor(day27$Genotype, levels = c("WT","kanttarelli","RNAi2","RNAi60"),
                        labels = c("WT","italic(kanttarelli)","italic(RNAi2)","italic(RNAi60)"))
day27$`Time (min)` = factor(day27$`Time (min)`, levels = c("0", "0.5", "1", "1.5", "2", "2.5", "3", "3.5", "4", "4.5", 
                                                           "5", "5.5", "6", "6.5", "7", "7.5", "8", "8.5", "9", "9.5", "10", 
                                                           "10.5", "11", "11.5", "12", "12.5", "13", "13.5", "14", "14.5", 
                                                           "15", "15.5", "16", "16.5", "17", "17.5", "18", "18.5", "19", 
                                                           "20-22"))
df.summary.day27 <- day27 %>%
  group_by(Genotype, `Time (min)`) %>%
  summarise(
    sd = sd(`germination ratio`, na.rm = TRUE),
    ger = mean(`germination ratio`)
  )
df.summary.day27[is.na(df.summary.day27)] <- 0
df.summary.day27

ggplot(df.summary.day27,  aes(x=`Time (min)`, y=`ger`, fill=Genotype)) + 
  geom_bar(stat='identity') +
  geom_errorbar(aes(ymin=ifelse(ger-sd<0,0, ger-sd),ymax=ger+sd),width=0.2) +
  facet_wrap(~Genotype, labeller = label_parsed) +
  scale_fill_manual(values = c(WT = "#1b7837",
                               "italic(RNAi2)" = "#e7298a", 
                               "italic(RNAi60)" = "#d95f02",
                               "italic(kanttarelli)" = "#762a83")) +
  scale_x_discrete(breaks = function(x){x[c(TRUE, FALSE, FALSE,FALSE,FALSE)]}) +
  ylim(0,100) +
  labs(x = "Time (min)", y = "Germinatiom (%) of O.minor") +
  theme_bw() +
  theme(axis.title.y = element_text(size=14, face="bold"),
        axis.text.x = element_text(),
        axis.text=element_text(size=12),
        strip.text = element_text(size = 12),
        legend.position="none")
ggsave(filename="O.minor germination ratio day27.png", width = 6.3, height = 5, unit = "in", device='png', dpi=700)


## control
control = read_excel("Supporting Data 04 O.minor germination assay.xlsx", sheet = "Control")
colnames(control)[1] = "Genotype"
control$Genotype[control$Genotype == "tap water"] <- "water"
control$Genotype = factor(control$Genotype, levels = c("GR24","water"))

df.summary.control <- control %>%
  group_by(Genotype,control) %>%
  summarise(
    sd = sd(`germination ratio`, na.rm = TRUE),
    ger = mean(`germination ratio`)
  )

ggplot(df.summary.control,  aes(x=control, y=`ger`, fill=Genotype)) + 
  geom_bar(stat='identity', width=0.15) +
  geom_errorbar(aes(ymin=ifelse(ger-sd<0,0, ger-sd),ymax=ger+sd),width=0.03) +
  facet_wrap(~Genotype, nrow = 2) +
  scale_fill_manual(values = c(GR24 = "#525252",
                               `tap water` = "#252525")) +
  ylim(0,100) +
  labs(x = "", y = "") +
  theme_bw() +
  theme(axis.ticks.y = element_blank(),
        axis.text.y=element_blank(),
        axis.text=element_text(size=12),
        strip.text = element_text(size = 12),
        legend.position="none")
ggsave(filename="O.minor germination control.png", width = 1, height = 5, unit = "in", device='png', dpi=700)

