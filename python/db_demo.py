from tsdb1 import Pager


def main():
    pager = Pager()
    pager.open("test.tsdb1")
    page2 = pager.new_page()
    print(page2)
    pager.close()


if __name__ == "__main__":
    main()
