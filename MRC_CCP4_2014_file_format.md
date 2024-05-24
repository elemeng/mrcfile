# MRC File Header Documentation

This page gives the detailed specification of the MRC2014 format, as published by Cheng et al. (2015). For other versions of the MRC format and an overview of the format update process, see the parent page.

CCP-EM maintain a Python library for reading, writing and validating MRC2014 files. There is also a web server to validate files against the format specification given here.

Official website: https://www.ccpem.ac.uk/mrc_format/mrc2014.php

## Main Header Structure

The main header of an MRC file is 1024 bytes long, organized as 56 4-byte words followed by space for 10 80-byte text labels.

| Word   | Bytes    | Variable Name      | Description                                                    |
| ------ | -------- | ------------------ | -------------------------------------------------------------- |
| 1      | 1-4      | NX or NC           | Number of columns in 3D data array (fast axis)                 |
| 2      | 5-8      | NY or NR           | Number of rows in 3D data array (medium axis)                  |
| 3      | 9-12     | NZ or NS           | Number of sections in 3D data array (slow axis)                |
| 4      | 13-16    | MODE               | Data type (see Note 2)                                         |
| 5      | 17-20    | NXSTART or NCSTART | Location of first column in unit cell                          |
| 6      | 21-24    | NYSTART or NRSTART | Location of first row in unit cell                             |
| 7      | 25-28    | NZSTART or NSSTART | Location of first section in unit cell                         |
| 8      | 29-32    | MX                 | Sampling along X axis of unit cell (see Note 3)                |
| 9      | 33-36    | MY                 | Sampling along Y axis of unit cell                             |
| 10     | 37-40    | MZ                 | Sampling along Z axis of unit cell (see Note 3)                |
| 11-13  | 41-52    | CELLA              | Cell dimensions in angstroms                                   |
| 14-16  | 53-64    | CELLB              | Cell angles in degrees                                         |
| 17     | 65-68    | MAPC               | Axis corresponding to columns (1,2,3 for X,Y,Z) (see Note 4)   |
| 18     | 69-72    | MAPR               | Axis corresponding to rows (1,2,3 for X,Y,Z)                   |
| 19     | 73-76    | MAPS               | Axis corresponding to sections (1,2,3 for X,Y,Z)               |
| 20     | 77-80    | DMIN               | Minimum density value (see Note 5)                             |
| 21     | 81-84    | DMAX               | Maximum density value                                          |
| 22     | 85-88    | DMEAN              | Mean density value                                             |
| 23     | 89-92    | ISPG               | Space group number (see Note 6)                                |
| 24     | 93-96    | NSYMBT             | Size of extended header (in bytes) (see Note 7)                |
| 25-49  | 97-196   | EXTRA              | Extra space (default is 0)                                     |
| 27     | 105      | EXTTYP             | Code for the type of extended header (see Note 8)              |
| 28     | 109      | NVERSION           | Version of the MRC format (see Note 9)                         |
| 50-52  | 197-208  | ORIGIN             | Phase origin (pixels) or origin of subvolume (A) (see Note 10) |
| 53     | 209-212  | MAP                | Character string 'MAP ' to identify file type                  |
| 54     | 213-216  | MACHST             | Machine stamp encoding byte ordering of data (see Note 11)     |
| 55     | 217-220  | RMS                | RMS deviation of map from mean density                         |
| 56     | 221-224  | NLABL              | Number of labels being used                                    |
| 57-256 | 225-1024 | LABEL(20,10)       | 10 80-character text labels                                    |

## Extended header

In the original definition, the extended header holds space group symmetry records stored as text as in International Tables, operators separated by _ and grouped into 'lines' of 80 characters (ie. symmetry operators do not cross the ends of the 80-character 'lines' and the 'lines' do not terminate in a _). The extended header is now used by different software to hold various additional metadata instead, as indicated by the EXTTYP tag.

## Detailed Notes (continued)

**Note 1: Data Array Dimensions**

- Description: The data block of an MRC format file holds a 3D array of data (of type specified by MODE). NC, NR, NS specify the dimensions (in grid points) of this array, orientated according to MAPC/MAPR/MAPS. In EM, this will correspond to the dimensions of a volume/map, or the combined size of an image/volume stack. In crystallography, this will correspond to the dimensions of a map, which may cover a crystallographic unit cell or may cover some fraction or multiple of a unit cell.

**Note 2: Data Types (MODE)**

- Description: In the MRC2014 format, Mode 0 has been clarified as signed, and mode 6 has been added for 16-bit unsigned integer data. See updates page for additional modes.

**Note 3: Sampling Grid**

- Description: In crystallographic usage, MZ represents the number of intervals, or sampling grid, along Z in a crystallographic unit cell. This need not be the same as NZ (or NX/NY if axes permuted) if the map doesn't cover exactly a single unit cell. For microscopy, where there is no unit cell, MZ represents the number of sections in a single volume. For a volume stack, NZ/MZ will be the number of volumes in the stack. For images, MZ = 1.

**Note 4: Axis Correspondence (MAPC, MAPR, MAPS)**

- Description: In EM MAPC, MAPR, MAPS = 1,2,3 so that sections and images are perpendicular to the Z axis. In crystallography, other orderings are possible. For example, in some spacegroups it is convenient to section along the Y axis (i.e. where this is the polar axis).

**Note 5: Density Statistics**

- Description: Density statistics may not be kept up-to-date for image/volume stacks, since it is expensive to recalculate these every time a new image/volume is added/deleted. We have proposed the following convention: DMAX < DMIN, DMEAN < (smaller of DMIN and DMAX), RMS < 0 each indicate that the quantity in question is not well determined.

**Note 6: Space Group (ISPG)**

- Description: Spacegroup 0 implies a 2D image or image stack. For crystallography, ISPG represents the actual spacegroup. For single volumes from EM/ET, the spacegroup should be 1. For volume stacks, we adopt the convention that ISPG is the spacegroup number + 400, which in EM/ET will typically be 401.

**Note 7: Extended Header Size (NSYMBT)**

- Description: NSYMBT specifies the size of the extended header in bytes, whether it contains symmetry records (as in the original format definition) or any other kind of additional metadata.

**Note 8: Extended Header Type (EXTTYP)**

- Description: A code for the kind of metadata held in the extended header.

**Note 9: MRC Format Version (NVERSION)**

- Description: The version of the MRC format that the file adheres to, specified as a 32-bit integer and calculated as: Year \* 10 + version within the year (base 0).

**Note 10: Phase Origin or Subvolume Origin (ORIGIN)**

- Description: For transforms (Mode 3 or 4), ORIGIN is the phase origin of the transformed image in pixels. For other modes, ORIGIN specifies the real space location of a subvolume taken from a larger volume.

![Alt ExampleImage](https://www.ccpem.ac.uk/images/image_real_origin_definition_v2.png)

**Note 11: Machine Stamp (MACHST)**

- Description: Bytes 213 and 214 contain 4 'nibbles' (half-bytes) indicating the representation of float, complex, integer, and character datatypes. Bytes 215 and 216 are unused. The CCP4 library contains a general representation of datatypes, but in practice it is safe to use 0x44 0x44 0x00 0x00 for little endian machines, and 0x11 0x11 0x00 0x00 for big endian machines.
