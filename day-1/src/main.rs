use std::collections::HashSet;

fn main() {
    part1();
    part2();
}

fn part1() {
    
    let numbers = input();
    
    for number in &numbers {
        let remainder = 2020 - number;

        if numbers.contains(&remainder) {
            println!("{} x {} * {}", number, remainder, number * remainder);
            break;
        }
    }
}

fn part2() {

}


fn input() -> HashSet<u32> {
    let mut numbers = HashSet::new();

    numbers.insert(1575);
    numbers.insert(1593);
    numbers.insert(1583);
    numbers.insert(1609);
    numbers.insert(1835);
    numbers.insert(2008);
    numbers.insert(1638);
    numbers.insert(1396);
    numbers.insert(1833);
    numbers.insert(1524);
    numbers.insert(1778);
    numbers.insert(1574);
    numbers.insert(1653);
    numbers.insert(1962);
    numbers.insert(1831);
    numbers.insert(1557);
    numbers.insert(1847);
    numbers.insert(1587);
    numbers.insert(1876);
    numbers.insert(1914);
    numbers.insert(1565);
    numbers.insert(1585);
    numbers.insert(1713);
    numbers.insert(35);
    numbers.insert(1862);
    numbers.insert(1885);
    numbers.insert(1735);
    numbers.insert(1497);
    numbers.insert(1989);
    numbers.insert(1871);
    numbers.insert(1923);
    numbers.insert(1917);
    numbers.insert(1719);
    numbers.insert(1797);
    numbers.insert(1222);
    numbers.insert(1493);
    numbers.insert(1939);
    numbers.insert(1139);
    numbers.insert(1260);
    numbers.insert(1622);
    numbers.insert(1625);
    numbers.insert(1683);
    numbers.insert(1742);
    numbers.insert(1996);
    numbers.insert(1579);
    numbers.insert(1703);
    numbers.insert(1692);
    numbers.insert(1920);
    numbers.insert(1536);
    numbers.insert(1965);
    numbers.insert(1936);
    numbers.insert(1947);
    numbers.insert(1800);
    numbers.insert(1556);
    numbers.insert(1633);
    numbers.insert(1530);
    numbers.insert(1612);
    numbers.insert(1764);
    numbers.insert(1810);
    numbers.insert(1845);
    numbers.insert(1750);
    numbers.insert(1854);
    numbers.insert(1973);
    numbers.insert(1512);
    numbers.insert(1856);
    numbers.insert(1568);
    numbers.insert(1634);
    numbers.insert(1630);
    numbers.insert(1602);
    numbers.insert(1555);
    numbers.insert(1681);
    numbers.insert(1844);
    numbers.insert(1544);
    numbers.insert(1909);
    numbers.insert(1690);
    numbers.insert(1851);
    numbers.insert(1785);
    numbers.insert(863);
    numbers.insert(1866);
    numbers.insert(1988);
    numbers.insert(1715);
    numbers.insert(1881);
    numbers.insert(1570);
    numbers.insert(1380);
    numbers.insert(1956);
    numbers.insert(777);
    numbers.insert(1693);
    numbers.insert(1717);
    numbers.insert(1724);
    numbers.insert(1975);
    numbers.insert(790);
    numbers.insert(1484);
    numbers.insert(1822);
    numbers.insert(1922);
    numbers.insert(1963);
    numbers.insert(1741);
    numbers.insert(1809);
    numbers.insert(1896);
    numbers.insert(1837);
    numbers.insert(1980);
    numbers.insert(1244);
    numbers.insert(1832);
    numbers.insert(1834);
    numbers.insert(1643);
    numbers.insert(1775);
    numbers.insert(1818);
    numbers.insert(1503);
    numbers.insert(1802);
    numbers.insert(1957);
    numbers.insert(1174);
    numbers.insert(1826);
    numbers.insert(1649);
    numbers.insert(1941);
    numbers.insert(1571);
    numbers.insert(1930);
    numbers.insert(1629);
    numbers.insert(1502);
    numbers.insert(2002);
    numbers.insert(1700);
    numbers.insert(1880);
    numbers.insert(1723);
    numbers.insert(1803);
    numbers.insert(2007);
    numbers.insert(1543);
    numbers.insert(1872);
    numbers.insert(1993);
    numbers.insert(1740);
    numbers.insert(1919);
    numbers.insert(1688);
    numbers.insert(1067);
    numbers.insert(1680);
    numbers.insert(1580);
    numbers.insert(1558);
    numbers.insert(1772);
    numbers.insert(1694);
    numbers.insert(1480);
    numbers.insert(1257);
    numbers.insert(1796);
    numbers.insert(2001);
    numbers.insert(537);
    numbers.insert(1701);
    numbers.insert(1613);
    numbers.insert(1784);
    numbers.insert(1559);
    numbers.insert(1482);
    numbers.insert(1968);
    numbers.insert(1604);
    numbers.insert(983);
    numbers.insert(1842);
    numbers.insert(1817);
    numbers.insert(1850);
    numbers.insert(1788);
    numbers.insert(1982);
    numbers.insert(1535);
    numbers.insert(1615);
    numbers.insert(453);
    numbers.insert(1895);
    numbers.insert(1443);
    numbers.insert(1308);
    numbers.insert(1533);
    numbers.insert(1714);
    numbers.insert(1765);
    numbers.insert(1037);
    numbers.insert(1992);
    numbers.insert(1843);
    numbers.insert(1883);
    numbers.insert(1981);
    numbers.insert(1525);
    numbers.insert(1038);
    numbers.insert(1540);
    numbers.insert(1766);
    numbers.insert(1886);
    numbers.insert(1546);
    numbers.insert(1716);
    numbers.insert(810);
    numbers.insert(1899);
    numbers.insert(1708);
    numbers.insert(1508);
    numbers.insert(1870);
    numbers.insert(1051);
    numbers.insert(1867);
    numbers.insert(1840);
    numbers.insert(1617);
    numbers.insert(1726);
    numbers.insert(1566);
    numbers.insert(1616);
    numbers.insert(1948);
    numbers.insert(1771);
    numbers.insert(1627);
    numbers.insert(1994);
    numbers.insert(1486);
    numbers.insert(1913);
    numbers.insert(1600);
    numbers.insert(1983);
    numbers.insert(1501);
    numbers.insert(2003);
    numbers.insert(1667);
    numbers.insert(1620);
    numbers.insert(1943);
    numbers.insert(1674);

    return numbers;
}