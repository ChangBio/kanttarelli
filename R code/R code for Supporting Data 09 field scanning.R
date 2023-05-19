library(readxl)
library(tidyverse)
library(ggplot2)
library("RColorBrewer")

WT = read_excel("Supporting Data 09 field scanning.xlsx", sheet = "WT")
colnames(WT) = sub("#", "", colnames(WT))
WT$Age = factor(WT$Age, levels=c("3 years","6 years","10 years"))

kan = read_excel("Supporting Data 09 field scanning.xlsx", sheet = "kanttarelli")
colnames(kan) = sub("#", "", colnames(kan))
kan$Age = factor(kan$Age, levels=c("4 years","13 years"))


#### WT branches####  
  ## branch numbers
branches = WT %>% select(`1branches`:`4branches`, Age)
names(branches) <- c("1st_order","2nd_order","3rd_order","4th_order","Age")
bran.orders = branches %>% gather(Order,values, -Age)
bran.orders[bran.orders==0] <- NA  

WT.order =ggplot(bran.orders, aes(x=Order,y=values,col = Order)) +
  geom_boxplot(alpha=0.0) +
  geom_jitter(shape=1,size=1.2, position=position_jitter(0.2)) +
  scale_y_continuous(limits = c(0, 950)) +
  facet_wrap(~Age) +
  scale_color_brewer(palette="BrBG") +
  labs(x = "", y = "WT branch number") + theme_light() + 
  theme(axis.title.y = element_text(size=12, face="bold"),
        axis.title.x = element_text(size=10, face="bold"),
        axis.ticks.x = element_blank(),
        axis.text.x=element_blank(),
        legend.title = element_text(size=10))
WT.order
ggsave(WT.order,file="WT branch number.pdf", width = 7.3, height = 3.7)
ggsave(WT.order,file="WT branch number.png", width = 7.3, height = 3.7)

## WT branch length 
len = WT %>% select(`Ave1Len`:`Ave4Len`, Age)
names(len) <- c("1st_order","2nd_order","3rd_order","4th_order","Age")
len.orders = len %>% gather(Order,values, -Age)
len.orders[len.orders==0] <- NA  

WT.order.len =ggplot(len.orders, aes(x=Order,y=values,col = Order)) +
  geom_boxplot(alpha=0.0) +
  geom_jitter(shape=1,size=1.2, position=position_jitter(0.2)) +
  scale_y_continuous(limits = c(0, 1.2)) +
  facet_wrap(~Age) +
  scale_color_brewer(palette="BrBG") +
  labs(x = "", y = "WT average branch length (m) per tree") + theme_light() + 
  theme(axis.title.y = element_text(size=12, face="bold"),
        axis.title.x = element_text(size=10, face="bold"),
        axis.ticks.x = element_blank(),
        axis.text.x=element_blank(),
        legend.title = element_text(size=10))
WT.order.len
ggsave(WT.order.len,file="WT average length.png", width = 7.3, height = 3.7)
ggsave(WT.order.len,file="WT average length.pdf", width = 7.3, height = 3.7)

## WT branch length 
ang = WT %>% select(`Ave1Angle`:`Ave4Angle`, Age)
names(ang) <- c("1st_order","2nd_order","3rd_order","4th_order","Age")
ang.orders = ang %>% gather(Order,values, -Age)
ang.orders[ang.orders==0] <- NA  

WT.order.ang =ggplot(ang.orders, aes(x=Order,y=values,col = Order)) +
  geom_boxplot(alpha=0.0) +
  geom_jitter(shape=1,size=1.2, position=position_jitter(0.2)) +
  scale_y_continuous(limits = c(0,150)) +
  facet_wrap(~Age) +
  scale_color_brewer(palette="BrBG") +
  labs(x = "", y = "WT average branch angle (°) per tree") + theme_light() + 
  theme(axis.title.y = element_text(size=12, face="bold"),
        axis.title.x = element_text(size=10, face="bold"),
        axis.ticks.x = element_blank(),
        axis.text.x=element_blank(),
        legend.title = element_text(size=10))
WT.order.ang
ggsave(WT.order.ang,file="WT average angle.png", width = 7.3, height = 3.7)
ggsave(WT.order.ang,file="WT average angle.pdf", width = 7.3, height = 3.7)




#### kan branches####  
## branch numbers
mycolors = brewer.pal(11, "PuOr")[c(2,3,4,8,9,10,11)]
# my_ylab = expression(paste(italic("kanttarelli  "), "branch number"))

branches = kan %>% select(`1branches`:`7branches`, Age)
names(branches) <- c("1st_order","2nd_order","3rd_order","4th_order",
                     "5th_order","6th_order","7th_order","Age")
bran.orders = branches %>% gather(Order,values, -Age)
bran.orders[bran.orders==0] <- NA   

kan.order =ggplot(bran.orders, aes(x=Order,y=values,col = Order)) +
  geom_boxplot(alpha=0.0) +
  geom_jitter(shape=1,size=1.2, position=position_jitter(0.2)) +
  scale_y_continuous(limits = c(0, 950)) +
  facet_wrap(~Age) +
  scale_color_manual(values=mycolors) +
  labs(x = "", y = "kanttarelli branch number") + theme_light() + 
  theme(axis.title.y = element_text(size=12, face="bold"),
        axis.title.x = element_text(size=10, face="bold"),
        axis.ticks.x = element_blank(),
        axis.text.x=element_blank(),
        legend.title = element_text(size=8))
kan.order
ggsave(kan.order,file="kan branch number.pdf", width = 7, height = 3.7)
ggsave(kan.order,file="kan branch number.pdf", width = 7, height = 3.7)

## kan branch length 
len = kan %>% select(`Ave1Len`:`Ave6Len`, Age)
names(len) <- c("1st_order","2nd_order","3rd_order","4th_order",
                "5th_order","6th_order","Age")
len.orders = len %>% gather(Order,values, -Age)
len.orders[len.orders==0] <- NA  

kan.order.len =ggplot(len.orders, aes(x=Order,y=values,col = Order)) +
  geom_boxplot(alpha=0.0) +
  geom_jitter(shape=1,size=1.2, position=position_jitter(0.2)) +
  scale_y_continuous(limits = c(0, 1.2)) +
  facet_wrap(~Age) +
  scale_color_manual(values=mycolors) +
  labs(x = "", y = "kanttarelli average branch length (m) per tree") + theme_light() + 
  theme(axis.title.y = element_text(size=12, face="bold"),
        axis.title.x = element_text(size=10, face="bold"),
        axis.ticks.x = element_blank(),
        axis.text.x=element_blank(),
        legend.title = element_text(size=10))
kan.order.len
ggsave(kan.order.len,file="kan average length.png", width = 7, height = 3.7)


## kan branch angle 
ang = kan %>% select(`Ave1Angle`:`Ave6Angle`, Age)
names(ang) <- c("1st_order","2nd_order","3rd_order","4th_order",
                "5th_order","6th_order","Age")
ang.orders = ang %>% gather(Order,values, -Age)
ang.orders[ang.orders==0] <- NA  

kan.order.ang =ggplot(ang.orders, aes(x=Order,y=values,col = Order)) +
  geom_boxplot(alpha=0.0) +
  geom_jitter(shape=1,size=1.2, position=position_jitter(0.2)) +
  scale_y_continuous(limits = c(0,150)) +
  facet_wrap(~Age) +
  scale_color_manual(values=mycolors) +
  labs(x = "", y = "kanttarelli average branch angle (°) per tree") + theme_light() + 
  theme(axis.title.y = element_text(size=12, face="bold"),
        axis.title.x = element_text(size=10, face="bold"),
        axis.ticks.x = element_blank(),
        axis.text.x=element_blank(),
        legend.title = element_text(size=10))
kan.order.ang
ggsave(kan.order.ang,file="kan average angle.png", width = 7, height = 3.7)
ggsave(kan.order.ang,file="kan average angle.pdf", width = 7, height = 3.7)


#### plant height ####
height = rbind(WT %>% select(1,2,7), kan %>% select(1,2,7))
height$Age = factor(height$Age, levels=c("3 years","4 years","6 years","10 years","13 years"))
height$Genotype = factor(height$Genotype, levels=c("WT","kanttarelli"),
                         labels = c("WT","italic(kanttarelli)"))

pheight =ggplot(height, aes(x=Age,y=height,col = Genotype)) +
  geom_boxplot(alpha=0.0) +
  geom_jitter(shape=1,size=1.2, position=position_jitter(0.2)) +
  facet_grid(. ~ Genotype, scales = "free_x", space='free', labeller = labeller(Genotype = label_parsed)) +
  scale_color_manual( values = c(WT = "#1b7837",
                                 "italic(kanttarelli)" = "#762a83")) +
  labs(x = "", y = "Plant height (m)") + theme_light() + 
  theme(axis.title.y = element_text(size=12, face="bold"),
        axis.text.x = element_text(),
        axis.text=element_text(size=10),
        legend.position="none") 
pheight
ggsave(pheight,file="height.png", width = 4.5, height = 3.7)



#### total branch number #####
totbranches = rbind(WT %>% select(1,2,8), kan %>% select(1,2,8))
totbranches$Age = factor(totbranches$Age, levels=c("3 years","4 years","6 years","10 years","13 years"))
totbranches$Genotype = factor(totbranches$Genotype, levels=c("WT","kanttarelli"))

ptotbranches =ggplot(totbranches, aes(x=Age,y=branches,col = Genotype)) +
  geom_boxplot(alpha=0.0) +
  geom_jitter(shape=1,size=1.2, position=position_jitter(0.2)) +
  facet_grid(. ~ Genotype, scales = "free_x", space='free') +
  scale_color_manual( values = c(WT = "#1b7837",
                                 kanttarelli = "#762a83")) +
  labs(x = "", y = "Total branch number") + theme_light() + 
  theme(axis.title.y = element_text(size=12, face="bold"),
        axis.text.x = element_text(),
        axis.text=element_text(size=10),
        legend.position="none") 
ptotbranches
ggsave(ptotbranches,file="Total branch number.png", width = 4.5, height = 3.7)

#### total average branch length #####
avelen = rbind(WT %>% select(1,2,13), kan %>% select(1,2,16))
avelen$Age = factor(avelen$Age, levels=c("3 years","4 years","6 years","10 years","13 years"))
avelen$Genotype = factor(avelen$Genotype, levels=c("WT","kanttarelli"))

pavelen =ggplot(avelen, aes(x=Age,y=TotalLen,col = Genotype)) +
  geom_boxplot(alpha=0.0) +
  geom_jitter(shape=1,size=1.2, position=position_jitter(0.2)) +
  facet_grid(. ~ Genotype, scales = "free_x", space='free') +
  scale_color_manual( values = c(WT = "#1b7837",
                                 kanttarelli = "#762a83")) +
  labs(x = "", y = "Average branch length (m)") + theme_light() + 
  theme(axis.title.y = element_text(size=12, face="bold"),
        axis.text.x = element_text(),
        axis.text=element_text(size=10),
        legend.position="none") 
pavelen
ggsave(pavelen,file="Total Average branch length.png", width = 4.5, height = 3.7)

#### total average branch angle #####
aveang = rbind(WT %>% select(1,2,18), kan %>% select(1,2,23))
aveang$Age = factor(aveang$Age, levels=c("3 years","4 years","6 years","10 years","13 years"))
aveang$Genotype = factor(aveang$Genotype, levels=c("WT","kanttarelli"))

paveang =ggplot(aveang, aes(x=Age,y=AveAngle,col = Genotype)) +
  geom_boxplot(alpha=0.0) +
  geom_jitter(shape=1,size=1.2, position=position_jitter(0.2)) +
  facet_grid(. ~ Genotype, scales = "free_x", space='free') +
  scale_color_manual( values = c(WT = "#1b7837",
                                 kanttarelli = "#762a83")) +
  labs(x = "", y = "Average branch angle (°)") + theme_light() + 
  theme(axis.title.y = element_text(size=12, face="bold"),
        axis.text.x = element_text(),
        axis.text=element_text(size=10),
        legend.position="none") 
paveang
ggsave(paveang,file="Total Average branch angle.png", width = 4.5, height = 3.7)

#### crown diameter ####
crownDiameter = rbind(WT %>% select(1,2,23), kan %>% select(1,2,30))
crownDiameter$Age = factor(crownDiameter$Age, levels=c("3 years","4 years","6 years","10 years","13 years"))
crownDiameter$Genotype = factor(crownDiameter$Genotype, levels=c("WT","kanttarelli"))

pcrownDiameter =ggplot(crownDiameter, aes(x=Age,y=CrownDiam,col = Genotype)) +
  geom_boxplot(alpha=0.0) +
  geom_jitter(shape=1,size=1.2, position=position_jitter(0.2)) +
  facet_grid(. ~ Genotype, scales = "free_x", space='free') +
  scale_color_manual( values = c(WT = "#1b7837",
                                 kanttarelli = "#762a83")) +
  labs(x = "", y = "Crown diameter (m)") + theme_light() + 
  theme(axis.title.y = element_text(size=12, face="bold"),
        axis.text.x = element_text(),
        axis.text=element_text(size=10),
        legend.position="none") 
pcrownDiameter
ggsave(pcrownDiameter,file="Total Average crown diameter.png", width = 4.5, height = 3.7)

#### crown area ####
crownArea = rbind(WT %>% select(1,2,24), kan %>% select(1,2,31))
crownArea$Age = factor(crownArea$Age, levels=c("3 years","4 years","6 years","10 years","13 years"))
crownArea$Genotype = factor(crownArea$Genotype, levels=c("WT","kanttarelli"))

pcrownArea =ggplot(crownArea, aes(x=Age,y=CrownArea,col = Genotype)) +
  geom_boxplot(alpha=0.0) +
  geom_jitter(shape=1,size=1.2, position=position_jitter(0.2)) +
  facet_grid(. ~ Genotype, scales = "free_x", space='free') +
  scale_color_manual( values = c(WT = "#1b7837",
                                 kanttarelli = "#762a83")) +
  labs(x = "", y = "Crown area (m2)") + theme_light() + 
  theme(axis.title.y = element_text(size=12, face="bold"),
        axis.text.x = element_text(),
        axis.text=element_text(size=10),
        legend.position="none") 
pcrownArea
ggsave(pcrownArea,file="Total Average crown area.png", width = 4.5, height = 3.7)


#### crown volumn ####
crownVolumn = rbind(WT %>% select(1,2,25), kan %>% select(1,2,32))
crownVolumn$Age = factor(crownVolumn$Age, levels=c("3 years","4 years","6 years","10 years","13 years"))
crownVolumn$Genotype = factor(crownVolumn$Genotype, levels=c("WT","kanttarelli"))

pcrownVolumn =ggplot(crownVolumn, aes(x=Age,y=CrownVol,col = Genotype)) +
  geom_boxplot(alpha=0.0) +
  geom_jitter(shape=1,size=1.2, position=position_jitter(0.2)) +
  facet_grid(. ~ Genotype, scales = "free_x", space='free') +
  scale_color_manual( values = c(WT = "#1b7837",
                                 kanttarelli = "#762a83")) +
  labs(x = "", y = "Crown volumn (m3)") + theme_light() + 
  theme(axis.title.y = element_text(size=12, face="bold"),
        axis.text.x = element_text(),
        axis.text=element_text(size=10),
        legend.position="none") 
pcrownVolumn
ggsave(pcrownVolumn,file="Total Average crown volumn.png", width = 4.5, height = 3.7)

