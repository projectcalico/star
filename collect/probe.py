import sys
import requests


def run():
    # Get args
    name = sys.argv[0]
    url = sys.argv[1]
    
    # Send request.
    try:
        r = requests.get(url, timeout=4)
    except Exception, e:
        print "ERROR: %s" % e
        sys.exit(2)

    # Print the raw response to stdout and return 
    # with an appropriate error code.
    print r.text
    if r.status_code != 200:
        sys.exit(1)
    else:
        sys.exit(0)


if __name__ == "__main__":
    run()
