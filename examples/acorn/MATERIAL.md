
## Blender Setup

#### Attributes

- Add a Vertex Group for the mesh, call it anything
- Rename the existing UV Map to "UVMap1", and add a new one "UVMap1"
- Add these custom Attributes:
  - _ATLAS_BLEND, Vertex, Float
  - _ATLAS_INDEX_0, Vertex, Integer
  - _ATLAS_INDEX_1, Vertex, Integer
  - VertexColor, Face Corner, Byte Color

#### Shader Nodes

Requires each of the 4 array texture layers as a separate image file.

Instantiate this twice:
- UV Map node, value UVMapN
- UV connection fed to 4 Image Texture nodes, choosing image layer files indexed 0 to 3 top to bottom
- Attribute node, value _ATLAS_INDEX_N
- 4 Image Texture and the Attribute nodes fed as 5 inputs to a node group

The node group contains:
- Four compare-and-mix setups, assuming N of 3 down to 0:
  - Math node with function Compare, with the last node group input fed as the first value, with second value N and epsilon 0.5
  - Base Color N into input B of a Mix Color node, with factor taken from the Compare node
  - Input A of the Mix Color node taken from the output of the Mix Color node for N + 1, or no input where N = 3
- The result from the N = 0 Mix Color node sent as the single node group output

And to pull the node groups together:
- Attribute node, value _ATLAS_BLEND
- Mix Color node with outputs of node groups 0 and 1, with factor from the blend attribute
- Color Attribute node, value VertexColor
- Mix node, of format Color, blend mode Multiply, and factor set to 1.0, with inputs taken from the Mix Color output and the VertexColor attribute
- Mix result fed into Principled BSDF
- BSDF output fed to Material Output

#### Export Settings

- Format: .glb
- Include: None
- Transform: +Y Up
- Data: Mesh UVs + Normals + Attributes, Material No export, Lighting Mode Standard

NOTE: Integer attributes are exported by the gltTF exporter as 32-bit floats. glTF doesn't support
32-bit signed integers, and requires attributes to be aligned to 4-byte boundaries. Using an integer
attribute in Blender and reading it as a float in a shader is a "best we can do" approach.
