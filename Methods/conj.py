import numpy as np

def conj(A, b, x):
    """
    The Conjugate Gradient Method
    Code adapted from Prof. Shewchuck 
    https://www.cs.cmu.edu/~quake-papers/painless-conjugate-gradient.pdf
    """  
    p = r_value = b - np.dot(A, x)
    rs = np.dot(np.transpose(r_value), r_value)
    
    for _ in range(len(b)):
        Ap = np.dot(A, p)
        alpha = rs / np.dot(np.transpose(p), Ap)
        x = x + np.dot(alpha, p)
        r = r - np.dot(alpha, Ap)
        r_update = np.dot(np.transpose(r), r)
        if np.sqrt(r_update) < 0.00000001:
            break
        p = r + (r_update/rs)*p
        rs = r_update
    return x