# Ascii-Factory
A WIP factory game where you turn binary 1s and 0s into ascii characters. Inspired by games like Dwarf Fortress and Factorio

# Game plan
  * A map is generated using noise to create "ore" zones that contain 1s and 0s
  * These 1s and 0s (referred to 'bits' from here on) can be mined and placed onto conveyer belts to then be processed into ascii characters
  * To process the bits into ascii characters, they must be fed in the correct order. Processers will try to create a character from the last 8 inserted bits since ascii characters are 8bit
    * For example to create the ascii 'A' character, you must feed bits into a processer in the order 01000001
  * To create/craft new stuff (sorters to sort bits, etc), you must feed in the name of what you want to craft into a crafter using ascii characters you have made via processing (would like criticism on this idea)
  
