
---
jupytext:
  text_representation:
    extension: .md
    format_name: myst
    format_version: 0.13
kernelspec:
  display_name: Python 3 (ipykernel)
  language: python
  name: python3
---

+++ {"nbgrader": {"grade": false, "grade_id": "cell-19c6eb4994e2384a", "locked": true, "schema_version": 3, "solution": false}}

# Manipuler des images

+++ {"nbgrader": {"grade": false, "grade_id": "cell-19c6eb4994e2384b", "locked": true, "schema_version": 3, "solution": false, "task": false}}

In this sheet, you will learn to perform some
simple manipulations and treatments on images. We will
start by practicing on a colorful image (source:

[wikimedia](https://commons.wikimedia.org/wiki/File:Apple_icon_2.png)):

:::{figure} media/apple.png
:alt: media/apple.png
:width: 40px
:align: center
:::

For this, we load it with the `PIL` (Python
Imaging Library) library specifying the name of the file containing it, then

l'affectons à une variable `img` pour pouvoir la manipuler par la
suite:

```{code-cell} ipython3
---
nbgrader:
  grade: false
  grade_id: cell-38a01921463de697
  locked: true
  schema_version: 3

  solution: false
---
from PIL import Image
```

```{code-cell} ipython3
---
nbgrader:
  grade: false
  grade_id: cell-b5659c2e482c3848

  locked: true
  schema_version: 3
  solution: false
---
img = Image.open("media/apple.png")
```

+++ {"nbgrader": {"grade": false, "grade_id": "cell-e75aecf3bd8946db", "locked": true, "schema_version": 3, "solution": false}}

Here is this image:

```{code-cell} ipython3
---
nbgrader:
  grade: false
  grade_id: cell-5f412c59d2396365
  locked: true
  schema_version: 3
  solution: false
---

img
```

+++ {"nbgrader": {"grade": false, "grade_id": "cell-450a9499627740e6", "locked": true, "schema_version": 3, "solution": false}}

To display it with axes and -- when the image has a low
resolution -- better identify individual pixels, you can use
`matplotlib`:

```{code-cell} ipython3

---
nbgrader:
  grade: false
  grade_id: cell-1b1461380b6fef35
  locked: true
  schema_version: 3
  solution: false
---
import matplotlib.pyplot as plt
```
```{code-cell} ipython3
---
nbgrader:
  grade: false
  grade_id: cell-1b1461380b6fef36
  locked: true
  schema_version: 3
  solution: false
---

plt.imshow(img);
```

+++ {"nbgrader": {"grade": false, "grade_id": "cell-b74d7516557d88dd", "locked": true, "schema_version": 3, "solution": false, "task": false}}

:::{hint} Traditional interface and object of matplotlib

Why a `;` at the end of the previous command? Because
`plt.imshow` doesn't return an image, but displays it as a side effect. The `;` prevents the display of what `plt.imshow` actually returns
</document>
(an object of type figure).

This somewhat dated approach is traditional in systems
like `Matlab`. The `matplotlib.pyplot` library reproduced it to
facilitate migration from these systems. By habit
many examples on the internet still use this approach; this
can remain practical as a shortcut in one-line examples
like above.

But we know since -- and this is what we teach you from the
beginning of the year -- that allows us to obtain much more modular code
if we properly separate the processing and calculations (for example
building a figure) from the inputs and outputs (for example displaying the
figure).

For this reason, for any non-trivial usage, it is preferable to use
the object interface of `matplotlib`, as in the following example:
    
:::

---
nbgrader:
  grade: false
  grade_id: cell-811d54735c3ff4e5
  locked: true
  schema_version: 3
  solution: false
  task: false
---

from matplotlib.figure import Figure
```

```nbgrader:
  grade: false
  grade_id: cell-0c1a66846231a7bb
  locked: true
  schema_version: 3
</translated>
```
  solution: false
  task: false
---
fig = Figure()              # Construction d'une nouvelle figure
subplot = fig.add_subplot() # Ajout d'une zone de dessin (appelée «axes» dans matplotlib) à la figure
subplot.imshow(img)         # Ajout d'une image à la zone de dessin
fig                         # Affichage de la figure
```

+++ {"nbgrader": {"grade": false, "grade_id": "cell-67d310808fac6650", "locked": true, "schema_version": 3, "solution": false}}

Consult the **PIL Image** documentation on the internet to find
how to get the width and height of this image. Store the
result in variables `width` and `height` and verify the
consistency with the figure above.

```{code-cell} ipython3
---
nbgrader:
  grade: false
</translated>
grade_id: cell-46598b84e0c79fc6
locked: false
schema_version: 3
solution: true
task: false
---
### BEGIN SOLUTION
width, height = img.size 
### END SOLUTION

```{code-cell} ipython3
---
nbgrader:
  grade: true
  grade_id: cell-c6bfc2a73d6866ce
  locked: true
  points: 1
  schema_version: 3
  solution: false
</translated>
  task: false
---
assert width == 256
assert height == 256
```

+++ {"nbgrader": {"grade": false, "grade_id": "cell-5aa2ff2d91d5b0b3", "locked": true, "schema_version": 3, "solution": false}}

## Images as tables


On souhaite maintenant pouvoir accéder au contenu de l'image pour
pouvoir calculer avec. Pour cela, nous allons convertir l'image en un
tableau de nombres `NumPy`, tels ceux que nous avons manipulés dans la
[fiche précédente](1_tableaux.md).

Voici le tableau associé à l'image:

```{code-cell} ipython3
---
nbgrader:
```
  grade: false
  grade_id: cell-8f62152c1e513665
  locked: true
  schema_version: 3
  solution: false
  task: false
---
import numpy as np
```
```{code-cell} ipython3
---
nbgrader:
  grade: false
  grade_id: cell-f1cf391e96f32f52
  locked: true
  schema_version: 3
  solution: false
---
M = np.array(img)

+++ {"nbgrader": {"grade": true, "grade_id": "cell-1a34fbbc93618b40", "locked": false, "schema_version": 3, "solution": true, "task": false, "points": 0}}

Referring to the course, how many rows, columns, and layers should this table have?

BEGIN SOLUTION

It should have 256 rows, 256 columns, and four layers.

nbgrader:
  grade: false
  grade_id: cell-f3258fc6004a71d6

  locked: false
  schema_version: 3
  solution: true
---
### BEGIN SOLUTION
M.shape
### END SOLUTION
```
Why four layers? Red, Green, Blue, ... and transparency!

+++ {"nbgrader": {"grade": false, "grade_id": "cell-397f146412f6fb76", "locked": true, "schema_version": 3, "solution": false}}

### Understand the color layers

As always, to better understand data, you need to
visualize it! Here's a figure representing our image and its three
color layers, red, green, blue. Observe how the colors of the image

de départ (blanc, vert, noir, rouge) se décomposent dans les
différentes couches.

```{code-cell} ipython3
---
nbgrader:
  grade: false
  grade_id: cell-49b556a7e4717b09
  locked: true
  schema_version: 3
```
  solution: false
  task: false
---
# Color scales (colormap) from black to the corresponding primary color
from matplotlib.colors import LinearSegmentedColormap
black_red_cmap   = LinearSegmentedColormap.from_list('black_red_cmap',   ["black", "red"])
black_green_cmap = LinearSegmentedColormap.from_list('black_green_cmap', ["black", "green"])
black_blue_cmap  = LinearSegmentedColormap.from_list('black_blue_cmap',  ["black", "blue"])

fig = Figure(figsize=(30, 5));

(subplot, subplotr, subplotg, subplotb) = fig.subplots(1, 4)  # Four drawing areas
# Drawing of the image and its three layers
subplot.imshow(M)
imgr = subplotr.imshow(M[:,:,0], cmap=black_red_cmap,   vmin=0, vmax=255)
imgg = subplotg.imshow(M[:,:,1], cmap=black_green_cmap, vmin=0, vmax=255)
imgb = subplotb.imshow(M[:,:,2], cmap=black_blue_cmap,  vmin=0, vmax=255)
# Adding color scale bars to the images
fig.colorbar(imgr, ax=subplotr);
fig.colorbar(imgg, ax=subplotg);
fig.colorbar(imgb, ax=subplotb);

fig
```

+++ {"nbgrader": {"schema_version": 3, "solution": false, "grade": false, "locked": true, "task": false, "grade_id": "cell-953569768ed43403"}}

Subsequently, we will visualize many images.  It is
therefore time to automate the construction of the figure above.
Par la suite, nous visualiserons de même de nombreuses images.  Il est
therefore time to automate the construction of the figure above.

:::{admonition} Exercise


Ouvrez le fichier [utilities.py](utilities.py) et complétez-y la
fonction `show_color_channels` à partir du code ci-dessus.

:::

```{code-cell} ipython3
---
nbgrader:
  grade: false
  grade_id: cell-40f2b1ca26aac943
```

  locked: true
  schema_version: 3
  solution: false
  task: false
---
# Automatically reload code when changes are made
%load_ext autoreload
%autoreload 2
from intro_science_donnees import *
from utilities import *

```

```{code-cell} ipython3
---
nbgrader:
  grade: false
  grade_id: cell-f01f30acda19b78b
  locked: true
  schema_version: 3
  solution: false
```

  task: false
---
show_source(show_color_channels)
```

```{code-cell} ipython3
---
nbgrader:
  grade: true
  grade_id: cell-66040947f316edfb

  locked: true
  points: 1
  schema_version: 3
  solution: false
  task: false
---
show_color_channels(img)
```

+++ {"nbgrader": {"schema_version": 3, "solution": false, "grade": false, "locked": true, "task": false, "grade_id": "cell-fda75a63d59e2209"}}

Verification: `show_color_channels` correctly returns a figure

```{code-cell} ipython3
---
nbgrader:
  grade: true
  grade_id: cell-80328561c59cf158
  locked: true
  points: 1
</document>
schema_version: 3
solution: false
task: false
---
assert isinstance(show_color_channels(img), Figure)
```

+++ {"nbgrader": {"schema_version": 3, "solution": false, "grade": false, "locked": true, "task": false, "grade_id": "cell-0cf719f423e37ef5"}}

Now let's look at the images from the week's dataset.

dernière:

```{code-cell} ipython3
---
nbgrader:
  grade: false
  grade_id: cell-1c62320d37fe2e8f
  locked: true
  schema_version: 3
  solution: false

  task: false
---
import os.path
dataset_dir = os.path.join(data.dir, 'ApplesAndBananasSimple')
images = load_images(dataset_dir, "*.png")
```

```{code-cell} ipython3
---
nbgrader:
```
  grade: false
  grade_id: cell-a7f1576c7b9d8917
  locked: true
  schema_version: 3
  solution: false
  task: false
---
image_grid(images, titles=images.index)

+++ {"nbgrader": {"schema_version": 3, "solution": true, "grade": true, "locked": false, "task": false, "grade_id": "cell-9f449b989ea74be6", "points": 0}}

:::{admonition} Exercise

Observe the following image and its layers. Explain what you
see. Try other examples.

BEGIN SOLUTION

We notice that the black pixels are at zero in the three layers and
On remarque que les pixels noirs sont à zéro dans les trois couches et

les pixels blancs à 255 dans les trois couches. Le jaune étant la
couleur complémentaire du bleu est à 255 en rouge et en vert et 0 en
bleu.

END SOLUTION

:::

```{code-cell} ipython3
img = images[10]
```
show_color_channels(img)
```

+++ {"nbgrader": {"schema_version": 3, "solution": false, "grade": false, "locked": true, "task": false, "grade_id": "cell-291f681a5aab593d"}}

We will now observe the **color histogram**
appearing in an image, using the
`color_histogram` utility
(as usual, you can consult the documentation and code by introspection with
`color_histogram?` and
`color_histogram??`):

```{code-cell} ipython3
---
nbgrader:
  grade: false
  grade_id: cell-ed77018895a029a9
  locked: true
  schema_version: 3
  solution: false
  task: false
```
color_histogram(img)
```

+++ {"nbgrader": {"schema_version": 3, "solution": true, "grade": true, "locked": false, "task": false, "grade_id": "cell-a887ca1b2eac11df", "points": 2}}

:::{admonition} Exercise

Observe the histograms below of the tenth and third
image, and interpret them.

Image 10 : we notice that the green and red pixels are predominantly located
in values greater than 150, while many blue pixels are located in values less than. This
is logical given the yellow color of the apple (yellow being the
complementary color of blue).

Image 3 : we notice that there are more red pixels having the

valeur 256 ; bleus et verts ayant dans l'ensemble des valeurs plus
faibles que les pixels rouges. Cohérent, puisque la pomme de l'image
est rouge.

:::

```{code-cell} ipython3
---
```
nbgrader:
  grade: false
  grade_id: cell-3828ddf071670eff
  locked: true
  schema_version: 3
  solution: false
  task: false
---
img = images[9]
show_color_channels(img)

```

```{code-cell} ipython3
---
nbgrader:
  grade: false
  grade_id: cell-3d6d2c2781569087
  locked: true
  schema_version: 3
  solution: false
</document>
</translated>
```
  task: false
---
color_histogram(img)
```

```
nbgrader:
  grade: false
  grade_id: cell-4f5ef22b7b70c0c8

  locked: true
  schema_version: 3
  solution: false
  task: false
---
img = images[2]
show_color_channels(img)
```

```{code-cell} ipython3

---
nbgrader:
  grade: false
  grade_id: cell-7ef43bfaf750914f
  locked: true
  schema_version: 3
  solution: false
  task: false
---
color_histogram(img)

+++ {"nbgrader": {"grade": false, "grade_id": "cell-5d6255f7be07beda", "locked": true, "schema_version": 3, "solution": false}}

## Color Separation

Nous allons maintenant extraire les trois canaux, rouge, vert,
bleu. Pour le canal des rouges, on extrait le sous-tableau à deux
dimensions de toutes les cases d'indice $(i,j,k)$ avec $k=0$. Le 
`* 1.0` sert à convertir les valeurs en nombres à virgule.

```{code-cell} ipython3
---
nbgrader:
  grade: false
  grade_id: cell-dfe51efcf6df748b
  locked: true
  schema_version: 3
  solution: false
---
```
R = M[:,:,0] * 1.0
```

+++ {"nbgrader": {"grade": false, "grade_id": "cell-59920f8c83495b86", "locked": true, "schema_version": 3, "solution": false}}

Looking at the result directly is not very informative:

```{code-cell} ipython3
---
nbgrader:
```
  grade: false
  grade_id: cell-79a4159977138616
  locked: true
  schema_version: 3
  solution: false
---
R
```

+++ {"nbgrader": {"grade": false, "grade_id": "cell-58b15c024481ac3d", "locked": true, "schema_version": 3, "solution": false}}

As usual, it is better to *visualize* it:

```{code-cell} ipython3
---
nbgrader:
  grade: false
  grade_id: cell-403d27ca87e158c3
  locked: true
  schema_version: 3

  solution: false
---
fig = Figure(figsize=(5,5))
ax, axr = fig.subplots(1,2)
ax.imshow(M)
axr.imshow(R, cmap='Greys_r', vmin=0, vmax=255)
fig
```

+++ {"nbgrader": {"grade": false, "grade_id": "cell-f5748504b4d52000", "locked": true, "schema_version": 3, "solution": false, "task": false}}

:::{admonition} Exercice

Extract the green and blue channels from the first image
into the `G` and `B` variables. Feel free to visualize them!

:::

```python
---
```
nbgrader:
  grade: false
  grade_id: cell-2793bd4b1f83558d
  locked: false
  schema_version: 3
  solution: true
  task: false
---
### BEGIN SOLUTION
G = M[:,:,1] * 1.0

fig = Figure(figsize=(5,5))
ax, axr = fig.subplots(1,2)
ax.imshow(M)
axr.imshow(G, cmap='Greys_r', vmin=0, vmax=255)
fig
### END SOLUTION

---
nbgrader:
  grade: true
  grade_id: cell-841034979ffe6094
  locked: true
  points: 1
  schema_version: 3
  solution: false
  task: false
---

assert G.shape == (256, 256)
assert abs(G.mean() - 158.27) < 0.1
```python
---
nbgrader:
  grade: false
  grade_id: cell-8baf6161cb011920
  locked: false
```

  schema_version: 3
  solution: true
  task: false
---
### BEGIN SOLUTION
B = M[:,:,2] * 1.0

fig = Figure(figsize=(5,5))
ax, axr = fig.subplots(1,2)
ax.imshow(M)

axr.imshow(B, cmap='Greys_r', vmin=0, vmax=255)
fig
### END SOLUTION
---
nbgrader:
  grade: true
  grade_id: cell-841034979ffe6095

  locked: true
  points: 1
  schema_version: 3
  solution: false
  task: false
---
assert B.shape == (256, 256)
assert abs(B.mean() - 148.39) < 0.1
```
+++ {"nbgrader": {"grade": false, "grade_id": "cell-233c14413c6e3e6f", "locked": true, "schema_version": 3, "solution": false}}

It is now easy to perform arithmetic on all the
pixels. For example the sum of the green and red intensities is written:

```{code-cell} ipython3
---
nbgrader:
  grade: false
  grade_id: cell-0bd25c9e1e6c6abf

  locked: true
  schema_version: 3
  solution: false
---
G + R
```

+++ {"nbgrader": {"grade": false, "grade_id": "cell-5d168111627796e1", "locked": true, "schema_version": 3, "solution": false}}

:::{admonition} Exercise

Calculate and visualize the brightness of all pixels of the image, the
*brightness* of a pixel $(r,g,b)$ being defined as the average
$v=\frac{r+g+b}{3}$:

:::

```{code-cell} ipython3
---
nbgrader:

  grade: false
  grade_id: cell-4956fa101f9c567c
  locked: false
  schema_version: 3
  solution: true
  task: false
---
### BEGIN SOLUTION
V = (R+G+B)/3

fig = Figure(figsize=(5,5))
ax, axr = fig.subplots(1,2)
ax.imshow(M)
axr.imshow(V, cmap='Greys_r', vmin=0, vmax=255)
fig
### END SOLUTION

nbgrader:
  grade: true
  grade_id: cell-841034979ffe6096
  locked: true
  points: 1
  schema_version: 3
  solution: false
  task: false
---
assert V.shape == (256, 256)

assert abs(V.mean() - 172.44) < 0.1
```

+++ {"nbgrader": {"grade": false, "grade_id": "cell-5d168111627796e2", "locked": true, "schema_version": 3, "solution": false}}

You have just converted the image to grayscale! Pour que cela
colle au mieux avec notre perception visuelle, il faudrait en fait
utiliser une moyenne légèrement pondérée; voir par exemple la
[Wikipedia](https://fr.wikipedia.org/wiki/Niveau_de_gris#Convertir_une_image_couleur_en_niveau_de_gris).


+++ {"nbgrader": {"grade": false, "grade_id": "cell-d0fb04b5c7bf1744", "locked": true, "schema_version": 3, "solution": false}}

## Conclusion

We have seen in this sheet how to load an image in Python
and perform some manipulations, visualizations and simple calculations
on it. This was the occasion to better understand the decomposition
of an image into layers of color.

:::{admonition} Exercise

Update your report, and especially the "code review" section
to check your utilities in [utilities.py](utilities.py).

:::

You can now move on to
the [attribute extraction](3_extraction_d_attributs.md)
