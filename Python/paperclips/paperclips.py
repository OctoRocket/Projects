from selenium import webdriver
from webdriver_manager.chrome import ChromeDriverManager

driver = webdriver.Chrome("./chromedriver")
driver.get("https://www.decisionproblem.com/paperclips/index2.html")
