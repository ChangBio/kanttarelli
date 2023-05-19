
library(readxl)
library(tidyverse)
library(ggplot2)

#### branching assay ####
branching = read_excel("Supporting Data 02 Arabidopsis complementary assay.xlsx", sheet = "T1branching")

branching$Genotype = factor(branching$Genotype, levels=c("Col","pBpMAX1::BpMAX1 (Col)","pBpMAX1::BpMAX1 (max1)","max1"))

bran = ggplot(branching, aes(x=Genotype, y=`Branch number`, fill=Genotype)) +
       geom_boxplot() +
       scale_fill_manual(values = c(Col = "#1b7837",
                                      `pBpMAX1::BpMAX1 (Col)` = "#ccece6", 
                                      `pBpMAX1::BpMAX1 (max1)` = "#66c2a4",
                                      max1 = "#762a83")) +
       labs(title = "", y = "Branch number") +
       scale_x_discrete(labels = c("Col" = "Col",
                                  "pBpMAX1::BpMAX1 (Col)" = "pBpMAX1::BpMAX1\n(Col)",
                                  "pBpMAX1::BpMAX1 (max1)" = "pBpMAX1::BpMAX1\n(max1)",
                                  "max1" = "max1")) +
       theme_classic() +
       theme(legend.position = "none",
             axis.title.x = element_blank(),
             axis.text.x = element_text(face="italic", angle = 20, vjust = 0.7))
bran
ggsave(bran, filename = "branching assay.png", width = 4, height = 3)


bran.aov <- aov(`Branch number` ~ Genotype, data = branching)
summary(bran.aov) #  5.21e-06 ***
TukeyHSD(bran.aov) # a, a, a, b
       
#### height assay ####
heightdata = read_excel("Supporting Data 02 Arabidopsis complementary assay.xlsx", sheet = "T1height")

heightdata$Genotype = factor(heightdata$Genotype, levels=c("Col","pBpMAX1::BpMAX1 (Col)","pBpMAX1::BpMAX1 (max1)","max1"))

height = ggplot(heightdata, aes(x=Genotype, y=`Plant height (cm)`, fill=Genotype)) +
          geom_boxplot() +
          scale_fill_manual(values = c(Col = "#1b7837",
                                       `pBpMAX1::BpMAX1 (Col)` = "#ccece6", 
                                       `pBpMAX1::BpMAX1 (max1)` = "#66c2a4",
                                       max1 = "#762a83")) +
          labs(title = "", y = "Plant height (cm)") +
          scale_x_discrete(labels = c("Col" = "Col",
                                      "pBpMAX1::BpMAX1 (Col)" = "pBpMAX1::BpMAX1\n(Col)",
                                      "pBpMAX1::BpMAX1 (max1)" = "pBpMAX1::BpMAX1\n(max1)",
                                      "max1" = "max1")) +
          theme_classic() +
          theme(legend.position = "none",
                axis.title.x = element_blank(),
                axis.text.x = element_text(face="italic", angle = 20, vjust = 0.7))
height
ggsave(height, filename = "height assay.png", width = 4, height = 3)
        
height.aov <- aov(`Plant height (cm)` ~ Genotype, data = heightdata)
summary(height.aov) #  2.01e-06 ***
TukeyHSD(height.aov) # a, a, a, b      
