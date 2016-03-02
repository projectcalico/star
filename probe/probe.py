import sys
import requests


def run():
    # Get args
    name = sys.argv[0]
    url = sys.argv[1]
    
    # Send request.
    print "Sending request: %s" % url
    try:
        r = requests.get(url, timeout=4)
    except Exception, e:
        print "Error: %s" % e
        sys.exit(2)

    print "Response: %s" % r

    if r.status_code != 200:
        sys.exit(1)
    
    return 0


if __name__ == "__main__":
    run()
