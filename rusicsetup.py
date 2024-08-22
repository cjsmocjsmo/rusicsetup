import os
import subprocess

CWD = os.getcwd()

def nfo_dir_check():
    if os.path.exists('/usr/share/rusicsetup/rusicsetup/nfo'):
        return True
    else:
        return False

def create_nfo_dir():
    if not os.path.exists('/usr/share/rusicsetup/rusicsetup/nfo'):
        os.makedirs('/usr/share/rusicsetup/rusicsetup/nfo')
        print('NFO directory created')
    else:
        print('NFO directory already exists')

def rm_nfo_dir():
    if os.path.exists('/usr/share/rusicsetup/rusicsetup/nfo'):
        subprocess.call(['rm', '-r', '/usr/share/rusicsetup/rusicsetup/nfo'])
        print('NFO files removed')

def db_dir_check():
    if os.path.exists('/usr/share/rusicsetup/rusicsetup/db'):
        return True
    else:
        return False

def create_db_dir():
    if not os.path.exists('/usr/share/rusicsetup/rusicsetup/db'):
        subprocess.call(['mkdir', '/usr/share/rusicsetup/rusicsetup/db'])
        print('DB dir not present, creating it')
        subprocess.call(['touch', '/usr/share/rusicsetup/rusicsetup/db/rusic.db'])
        print('DB file created')
    else:
        print('DB directory already exists')

def rm_db_dir():
    if os.path.exists('/usr/share/rusicsetup/rusicsetup/db'):
        subprocess.call(['rm', '-rf', '/usr/share/rusicsetup/rusicsetup/db'])
        print('DB files removed')

def thumb_dir_check():
    if os.path.exists('/usr/share/rusicsetup/rusicsetup/thumbs'):
        return True
    else:
        return False

def create_thumb_dir():
    if not os.path.exists('/usr/share/rusicsetup/rusicsetup/thumbs'):
        os.makedirs('/usr/share/rusicsetup/rusicsetup/thumbs')
        print('Thumbs directory created')
    else:
        print('Thumbs directory already exists')

def rm_thumb_dir():
    if os.path.exists('/usr/share/rusicsetup/rusicsetup/thumbs'):
        subprocess.call(['rm', '-rf', '/usr/share/rusicsetup/rusicsetup/thumbs'])
        print('Thumbs files removed')

def install_dir_check():
    if os.path.exists('/usr/share/rusicsetup/rusicsetup'):
        return True
    else:
        return False
    

if __name__ == '__main__':
    if install_dir_check():
        create_nfo_dir()
        create_db_dir()
        create_thumb_dir() 